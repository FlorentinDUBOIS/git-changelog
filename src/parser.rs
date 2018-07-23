//! # Parser
//!
//! Parser module parse git history tree in order to scrape information
use askama::Template;
use conf::Policy;
use failure::Error;
use git2::{Oid, Repository};
use time::{at_utc, now_utc, Timespec};

#[derive(Debug, Clone)]
pub struct Commit {
  pub hash: String,
  pub message: String,
  pub author: String,
  pub date: String,
}

#[derive(Debug, Clone)]
pub struct Repo {
  pub name: String,
  pub commits: Vec<Commit>,
}

impl Repo {
  pub fn new(name: &str, commits: &[Commit]) -> Self {
    Repo {
      name: String::from(name),
      commits: commits.to_owned(),
    }
  }
}

#[derive(Template, Debug, Clone)]
#[template(path = "changelog.md")]
pub struct Changelog {
  pub now: String,
  pub repositories: Vec<Repo>,
}

impl Changelog {
  pub fn new(now: &str, repositories: &[Repo]) -> Self {
    Changelog {
      now: String::from(now),
      repositories: repositories.to_owned(),
    }
  }
}

pub trait Parser {
  fn parse(&self) -> Result<Changelog, Error>;
}

#[derive(Debug, Clone)]
pub struct ParserImpl {
  pub no_merges: bool,
  pub policy: Policy,
}

impl Parser for ParserImpl {
  fn parse(&self) -> Result<Changelog, Error> {
    let now = now_utc().strftime("%F")?.to_string();

    let mut repositories = vec![];
    let paths = self.policy.repositories.paths.clone();
    for path in paths {
      let repository = Repository::discover(path.clone())?;
      let mut revwalk = repository.revwalk()?;

      revwalk.push_head()?;

      let mut commits = vec![];
      for oid in revwalk {
        if let Some(commit) = self.convert(&repository, &(oid?))? {
          commits.push(commit);
        }
      }

      let file_name = match path.file_stem() {
        Some(file_name) => file_name.to_str(),
        None => return Err(format_err!("Path should contains the project name")),
      };

      repositories.push(Repo::new(file_name.unwrap(), &commits));
    }

    Ok(Changelog::new(&now, &repositories))
  }
}

impl ParserImpl {
  pub fn new(no_merges: bool, policy: &Policy) -> ParserImpl {
    ParserImpl {
      no_merges,
      policy: policy.to_owned(),
    }
  }

  pub fn convert(&self, repository: &Repository, oid: &Oid) -> Result<Option<Commit>, Error> {
    let commit = repository.find_commit(*oid)?;
    let author = match commit.author().name() {
      Some(author) => String::from(author),
      None => match commit.committer().name() {
        Some(committer) => String::from(committer),
        None => String::new(),
      },
    };

    let message = match commit.summary() {
      Some(summary) => String::from(summary),
      None => match commit.message() {
        Some(message) => String::from(message),
        None => String::new(),
      },
    };

    if self.no_merges
      && (message.starts_with("Merge pull request") || message.starts_with("Merge branch"))
    {
      return Ok(None);
    }

    let time = commit.time();
    let tm = at_utc(Timespec::new(time.seconds(), 0));
    let mut hash = oid.to_string();

    hash.truncate(7);
    Ok(Some(Commit {
      hash,
      author,
      date: tm.strftime("%F")?.to_string(),
      message,
    }))
  }
}
