//! # Generate module
//!
//! The generate module provide the generate command.
use askama::Template;
use conf::Policy;
use failure::Error;
use parser::{Parser, ParserImpl};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// ## Generate command
///
/// Open the repository at the given `path`. Scrape git history information from it.
/// Output the result into the given `output` file.
pub fn generate(output: &PathBuf, no_merges: bool, policy: &PathBuf) -> Result<(), Error> {
  let policy = Policy::from(policy)?;

  let parser = ParserImpl::new(no_merges, &policy);
  let content = parser.parse()?.render()?;

  let mut file = File::create(output)?;

  file.write_all(content.as_bytes())?;
  file.flush()?;
  file.sync_all()?;

  Ok(())
}
