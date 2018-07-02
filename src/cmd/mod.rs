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
    /// Preset to use
    #[structopt(long = "preset", short = "p", default_value = "commitizen")]
    preset: String,

    /// Set the path of the git repository
    #[structopt(long = "path", default_value = ".", parse(from_os_str))]
    path: PathBuf,

    /// Set the path of the git repository
    #[structopt(long = "output", short = "o", default_value = "CHANGELOG.md", parse(from_os_str))]
    output: PathBuf,
  },
}
