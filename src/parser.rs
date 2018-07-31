//! # Parser module.
//!
//! The parser module provides traits and structures for generate changelog.
use askama::Template;
use failure::Error;
use git2;
use policy;
use regex::Regex;
use slog_scope::{logger, scope};
use std::collections::HashMap;
use std::convert::From;
use time::{at_utc, Timespec};

// https://regex101.com/r/X9RoUY/4
const PATTERN: &str =
    r"(?P<kind>[\w \-\./\\]+)(\((?P<scope>[\w \-\./\\]+)\))?: (?P<message>[\w \-\./\\]+)";

#[derive(Debug, Clone)]
struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

impl Commit {
    // Move this under the trait `std::convert::TryFrom<(&git2::Repository, &git2::Oid)>` when
    // feature will be stabilize.
    pub fn try_from(tuple: (&git2::Repository, &git2::Oid)) -> Result<Self, Error> {
        let (repository, oid) = tuple;
        let commit = repository.find_commit(oid.to_owned())?;
        let author = match commit.author().name() {
            Some(author) => String::from(author),
            None => match commit.committer().name() {
                Some(committer) => String::from(committer),
                None => return Err(format_err!("No such author or commiter")),
            },
        };

        let message = match commit.summary() {
            Some(summary) => String::from(summary),
            None => match commit.message() {
                Some(message) => String::from(message),
                None => return Err(format_err!("No such message or summary")),
            },
        };

        let hash = oid.to_string();
        let date = at_utc(Timespec::new(commit.time().seconds(), 0))
            .strftime("%F")?
            .to_string();

        Ok(Commit {
            hash,
            message: message.to_owned(),
            author,
            date,
        })
    }
}

#[derive(Debug, Clone)]
struct Kind {
    pub name: String,
    pub commits: Vec<Commit>,
}

impl From<(String, Vec<Commit>)> for Kind {
    fn from(tuple: (String, Vec<Commit>)) -> Kind {
        let (name, commits) = tuple;

        Kind { name, commits }
    }
}

#[derive(Debug, Clone)]
struct Repository {
    pub name: String,
    pub kinds: Vec<Kind>,
}

impl Repository {
    // Move this under the trait `std::convert::TryFrom<(&HashMap<String, String>, &policy::Repository)>`
    // when feature will be stabilize.
    pub fn try_from(tuple: (&HashMap<String, String>, &policy::Repository)) -> Result<Self, Error> {
        let (kinds, repository) = tuple;
        let repo = git2::Repository::discover(repository.path.to_owned())?;
        let mut revwalk = repo.revwalk()?;

        if let Some(ref range) = repository.range {
            revwalk.push_range(range.as_str())?;
        } else {
            revwalk.push_head()?;
        }

        let mut commits = HashMap::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = match Commit::try_from((&repo, &oid)) {
                Err(err) => {
                    error!("Could not parse the commit"; "hash" => oid.to_string(), "error" => err.to_string());
                    continue;
                }
                Ok(commit) => commit,
            };

            let Commit { hash, message, .. } = commit.to_owned();
            if message.starts_with("Merge pull request") || message.starts_with("Merge branch") {
                info!("Skip merge commit"; "hash" => hash);
                continue;
            }

            let re = Regex::new(PATTERN)?;
            if !re.is_match(&message) {
                error!("Could not parse the message"; "hash" => hash, "message" => message);
                continue;
            }

            let captures = re.captures(&message).unwrap();
            let kind = String::from(captures.name("kind").unwrap().as_str());
            let scope = match captures.name("scope") {
                Some(scope) => Some(String::from(scope.as_str())),
                None => None,
            };

            if !kinds.contains_key(&kind) {
                warn!("Kind is not contained in provided kinds"; "hash" => &hash, "kind" => kind);
                continue;
            }

            if let Some(ref scope) = scope {
                if let Some(ref scopes) = repository.scopes {
                    let _scopes = scope.as_str().split(',');
                    for _scope in _scopes {
                        if !scopes.contains(&String::from(_scope)) {
                            warn!("Scope is not contained in provided scopes";  "hash" => &hash, "scope" => scope);
                            continue;
                        }
                    }
                }
            }

            commits
                .entry(String::from(kinds.get(&kind).unwrap().as_str()))
                .or_insert_with(|| vec![])
                .push(commit);
        }

        let mut kinds = vec![];
        for (name, commits) in commits {
            kinds.push(Kind::from((name, commits)));
        }

        Ok(Repository {
            name: repository.name.to_owned(),
            kinds,
        })
    }
}

#[derive(Template, Debug, Clone)]
#[template(path = "changelog.md")]
struct Changelog {
    pub repositories: Vec<Repository>,
}

impl From<Vec<Repository>> for Changelog {
    fn from(repositories: Vec<Repository>) -> Changelog {
        Changelog { repositories }
    }
}

/// Define a trait to implement in order to be used
pub trait Parser {
    fn render(&self, policy: &policy::Policy) -> Result<String, Error>;
}

#[derive(Default, Debug, Clone)]
pub struct ParserImpl;

impl Parser for ParserImpl {
    fn render(&self, policy: &policy::Policy) -> Result<String, Error> {
        let mut repositories = vec![];
        for repository in policy.repositories.clone() {
            scope(
                &logger().new(o!("repository" => repository.name.to_owned())),
                || {
                    match Repository::try_from((&policy.kinds, &repository)) {
                        Err(err) => crit!("Could not parse repository"; "error" => err.to_string()),
                        Ok(repository) => repositories.push(repository),
                    };
                },
            )
        }

        Ok(Changelog::from(repositories).render()?)
    }
}

impl ParserImpl {
    pub fn new() -> Self {
        ParserImpl
    }
}
