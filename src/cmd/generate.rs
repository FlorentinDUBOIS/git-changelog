//! # Generate module
//!
//! The generate module provide the generate command.
use askama::Template;
use failure::Error;
use git2::Repository;
use parser::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// ## Generate command
///
/// Open the repository at the given `path`. Scrape git history information from it.
/// Output the result into the given `output` file.
pub fn generate(path: &PathBuf, output: &PathBuf) -> Result<(), Error> {
  let repository = Repository::discover(path)?;

  let parser = Parser::new();
  let changelog = parser.parse(&repository)?;

  let mut file = File::create(output)?;
  let content = changelog.render()?;

  file.write(content.as_bytes())?;
  file.flush()?;
  file.sync_all()?;

  Ok(())
}
