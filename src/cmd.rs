//! # Command module
//!
//! The command module provide command line interface utilities.
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use failure::Error;

use crate::parser::{Parser, ParserImpl};
use crate::policy::Policy;

include!("./version.rs");

#[derive(StructOpt, Clone, Debug)]
pub struct Args {
    /// Increase log verbosity
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: usize,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Clone, Debug)]
pub enum Command {
    /// Prints version information
    #[structopt(name = "version")]
    Version,

    /// Generate changelog
    #[structopt(name = "generate")]
    Generate {
        /// Set the path of the git repository
        #[structopt(
            long = "output",
            short = "o",
            default_value = "CHANGELOG.md",
            parse(from_os_str)
        )]
        output: PathBuf,

        /// Set the policy
        #[structopt(long = "policy", default_value = "policy.toml", parse(from_os_str))]
        policy: PathBuf,
    },
}

/// Display the version on the standard output.
pub fn version() -> Result<(), Error> {
    let name = env!("CARGO_PKG_NAME");
    let semver = env!("CARGO_PKG_VERSION");

    println!("{} {} {}", name, semver, GITHASH);
    println!("{} build date {}", name, BUILD_DATE);
    Ok(())
}

/// Generate the changelog.
pub fn generate(output: &PathBuf, policy: &PathBuf) -> Result<(), Error> {
    let policy = Policy::from(policy)?;

    let parser = ParserImpl::new();
    let content = parser.render(&policy)?;

    let mut file = File::create(output)?;

    file.write_all(content.as_bytes())?;
    file.flush()?;
    file.sync_all()?;

    Ok(())
}
