//! # Changelog
//!
//! Generate a changelog using the git commit history
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate askama;
extern crate git2;
extern crate time;

pub mod cmd;
pub mod parser;

use cmd::generate::generate;
use cmd::version::version;
use cmd::{Command, Opt};
use failure::Error;
use structopt::StructOpt;

pub fn main() -> Result<(), Error> {
  env_logger::try_init()?;

  let opt = Opt::from_args();
  let result = match opt.cmd {
    Some(cmd) => match cmd {
      Command::Generate {
        path,
        output,
        no_merges,
      } => generate(&path, &output, &no_merges),
      Command::Version { .. } => version(),
    },
    None => Ok(()),
  };

  if let Err(err) = result {
    error!("{}", err);
  }

  info!("{} halted!", env!("CARGO_PKG_NAME"));
  Ok(())
}
