//! # Command module
//!
//! The command module provide command line interface utilities.
pub mod generate;
pub mod version;

use std::path::PathBuf;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "changelog")]
pub struct Opt {
  #[structopt(subcommand)]
  pub cmd: Option<Command>,
}

#[derive(StructOpt, Clone, Debug)]
pub enum Command {
  /// Prints version information
  #[structopt(name = "version")]
  Version,

  #[structopt(name = "generate")]
  Generate {
    /// Set the policy
    #[structopt(long = "policy", default_value = "policy.toml", parse(from_os_str))]
    policy: PathBuf,

    /// Set the path of the git repository
    #[structopt(long = "output", short = "o", default_value = "CHANGELOG.md", parse(from_os_str))]
    output: PathBuf,

    /// Remove merge commits
    #[structopt(long = "no-merges")]
    no_merges: bool,
  },
}
