//! # Parser module
//!
//! The parser module will parse the git commit history to build changelog

use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use askama::Template;
use failure::{Error, ResultExt};
use git2 as git;
use regex::Regex;
use time::{at_utc, Timespec};

use strfmt::strfmt;

use crate::conf;
use crate::conf::Configuration;

// https://regex101.com/r/X9RoUY/4
const PATTERN: &str =
    r"(?P<kind>[\w \-\./\\]+)(\((?P<scope>[\w \-\./\\]+)\))?: (?P<message>[\w \-\./\\]+)";

#[derive(Clone, Debug)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub link: Option<String>,
}

impl TryFrom<(&conf::Repository, &git::Commit<'_>)> for Commit {
    type Error = Error;

    fn try_from(tuple: (&conf::Repository, &git::Commit<'_>)) -> Result<Self, Self::Error> {
        let (conf, commit) = tuple;
        let author = match commit.author().name() {
            Some(author) => String::from(author),
            None => match commit.committer().name() {
                Some(committer) => String::from(committer),
                None => return err!("No such author or commiter"),
            },
        };

        let message = match commit.summary() {
            Some(summary) => String::from(summary),
            None => match commit.message() {
                Some(message) => String::from(message),
                None => return err!("No such message or summary"),
            },
        };

        let mut hash = commit.id().to_string();
        let date = at_utc(Timespec::new(commit.time().seconds(), 0))
            .strftime("%F")?
            .to_string();

        let mut link = None;
        if let Some(ref layout) = conf.link {
            let mut vars = HashMap::new();

            vars.insert(String::from("hash"), hash.to_owned());

            link = Some(
                strfmt(layout, &vars)
                    .with_context(|err| format!("could not format commit link, {}", err))?,
            );
        }

        hash.truncate(7);

        ok!(Self {
            hash,
            message: message.to_owned(),
            author,
            date,
            link,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    pub commits: HashMap<String, Vec<Commit>>,
}

impl From<(String, HashMap<String, Vec<Commit>>)> for Tag {
    fn from(tuple: (String, HashMap<String, Vec<Commit>>)) -> Self {
        let (name, commits) = tuple;

        Self { name, commits }
    }
}

#[derive(Clone, Debug)]
pub struct Repository {
    pub name: String,
    pub tags: Vec<Tag>,
}

impl From<String> for Repository {
    fn from(name: String) -> Self {
        Repository {
            name,
            tags: Default::default(),
        }
    }
}

impl TryFrom<(&HashMap<String, String>, &conf::Repository)> for Repository {
    type Error = Error;

    fn try_from(tuple: (&HashMap<String, String>, &conf::Repository)) -> Result<Self, Self::Error> {
        let (kinds, conf) = tuple;
        let mut repository = Repository::from(conf.name.to_owned());
        let repo = git::Repository::discover(&conf.path).with_context(|err| {
            format!(
                "could not retrieve git repository at '{:?}', {}",
                conf.path, err
            )
        })?;

        // We should build a map(commit-id -> tag) before starting walking over the git commit history.
        //
        // The full explanation is here:
        // https://stackoverflow.com/questions/36528576/get-annotated-tags-from-revwalk-commit/36555358#36555358
        let mut tags = HashMap::new();
        for tag in repo
            .tag_names(None)
            .with_context(|err| format!("could not retrieve git tags, {}", err))?
            .iter()
            {
                let object = repo
                    .revparse_single(tag.expect("tag to be written in utf-8 compliant format"))
                    .with_context(|err| format!("could not retrieve object for tag, {}", err))?;

                let tag = match object.to_owned().into_tag() {
                    Ok(tag) => tag,
                    Err(_) => {
                        let mut hash = object.id().to_string();
                        hash.truncate(7);
                        warn!("could not cast object into tag"; "hash" => hash);
                        continue;
                    }
                };

                tags.insert(tag.target_id().to_string(), tag);
            }

        let mut revwalk = repo
            .revwalk()
            .with_context(|err| format!("could create a walker on git history, {}", err))?;

        match &conf.range {
            Some(range) => {
                revwalk
                    .push_range(&range)
                    .with_context(|err| format!("could not parse commit range, {}", err))?;
            }
            None => {
                revwalk
                    .push_head()
                    .with_context(|err| format!("could not push HEAD commit, {}", err))?;
            }
        }

        revwalk.set_sorting(git::Sort::TIME | git::Sort::REVERSE);

        let mut commits = HashMap::new();
        for oid in revwalk {
            let oid =
                oid.with_context(|err| format!("could not retrieve object identifier, {}", err))?;

            let commit = repo
                .find_commit(oid)
                .with_context(|err| format!("could not retrieve commit '{}', {}", oid, err))?;

            let commit = Commit::try_from((conf, &commit))
                .with_context(|err| format!("could not parse commit '{}', {}", oid, err))?;

            let Commit { hash, message, .. } = commit.to_owned();
            if message.starts_with("Merge pull request") || message.starts_with("Merge branch") {
                info!("Skip merge commit"; "hash" => &hash);
                continue;
            }

            let re = Regex::new(PATTERN).expect("pattern to be a valid regular expression");
            if !re.is_match(&message) {
                error!("Could not parse the message"; "hash" => hash, "message" => message);
                continue;
            }

            let captures = re.captures(&message).expect("captures to exists in PATTERN regex");
            let kind = String::from(
                captures
                    .name("kind")
                    .expect("To have 'kind' group in the PATTERN regex")
                    .as_str(),
            );

            let scope = match captures.name("scope") {
                Some(scope) => Some(String::from(scope.as_str())),
                None => None,
            };

            if !kinds.contains_key(&kind) {
                warn!("Kind is not contained in provided kinds"; "hash" => &hash, "kind" => kind);
                warn!("Skip commit"; "hash" => &hash);
                continue;
            }

            if let Some(ref scope) = scope {
                let sub_scopes = scope.as_str().split(',');
                if let Some(ref scopes) = conf.scopes {
                    for sub_scope in sub_scopes {
                        if !scopes.contains(&String::from(sub_scope)) {
                            warn!("Scope is not contained in provided scopes";  "hash" => &hash, "scope" => scope);
                            continue;
                        }
                    }
                }
            }

            (&mut commits)
                .entry(String::from(
                    kinds
                        .get(&kind)
                        .expect("To have 'kind' defined in repository's kinds")
                        .as_str(),
                ))
                .or_insert_with(|| vec![])
                .push(commit);

            if let Some(tag) = tags.get(&oid.to_string()) {
                repository
                    .tags
                    .push(Tag::from((String::from(tag.name().expect("tag name to be utf-8 compliant")), commits)));

                commits = HashMap::new();
            }
        }

        if !commits.is_empty() {
            repository
                .tags
                .push(Tag::from((String::from("Technical preview"), commits)));
        }

        repository.tags.reverse();

        ok!(repository)
    }
}

#[derive(Template, Default, Clone, Debug)]
#[template(path = "changelog.md", escape = "none")]
pub struct Changelog {
    pub repositories: Vec<Repository>,
}

impl TryFrom<Rc<Configuration>> for Changelog {
    type Error = Error;

    fn try_from(conf: Rc<Configuration>) -> Result<Self, Self::Error> {
        let mut changelog = Changelog::default();

        for repository in &conf.repositories {
            changelog.repositories.push(
                Repository::try_from((&conf.kinds, repository)).with_context(|err| {
                    format!(
                        "could not process repository '{}', {}",
                        repository.name, err
                    )
                })?,
            );
        }

        ok!(changelog)
    }
}
