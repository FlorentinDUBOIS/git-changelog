//! # Parser
//!
//! Parser module parse git history tree in order to scrape information
use askama::Template;
use failure::Error;
use git2::{Oid, Repository};
use time::{at_utc, Timespec};

#[derive(Debug, Clone)]
pub struct Commit {
  pub hash: String,
  pub message: String,
  pub author: String,
  pub date: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "changelog.md")]
pub struct Changelog {
  pub commits: Vec<Commit>,
}

impl Changelog {
  pub fn new(commits: &[Commit]) -> Changelog {
    Changelog {
      commits: commits.to_owned(),
    }
  }
}

pub trait Parser {
  fn parse(&self, repository: &Repository) -> Result<Changelog, Error>;
}

#[derive(Debug, Clone)]
pub struct ParserImpl {
  pub no_merges: bool,
}

impl Parser for ParserImpl {
  fn parse(&self, repository: &Repository) -> Result<Changelog, Error> {
    let mut revwalk = repository.revwalk()?;

    revwalk.push_head()?;

    let mut commits = vec![];
    for oid in revwalk {
      let oid = oid?;
      if let Some(commit) = self.convert(repository, &oid)? {
        commits.push(commit);
      }
    }

    Ok(Changelog::new(&commits))
  }
}

impl ParserImpl {
  pub fn new(no_merges: bool) -> ParserImpl {
    ParserImpl { no_merges }
  }
}

impl ParserImpl {
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
      && (message.starts_with("Merge pull request") || message.starts_with("Merge branches"))
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
