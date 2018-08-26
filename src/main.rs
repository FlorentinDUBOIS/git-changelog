//! # Changelog
//!
//! Generate a changelog using the git commit history
#![feature(try_from)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate askama;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;

pub mod cmd;
pub mod logger;
pub mod parser;
pub mod policy;

use crate::cmd::{Command, Opt};
use failure::Error;
use structopt::StructOpt;

pub fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let _guard = logger::initialize(opt.verbose)?;

    let result = match opt.cmd {
        Some(cmd) => match cmd {
            Command::Generate { output, policy } => cmd::generate(&output, &policy),
            Command::Version { .. } => cmd::version(),
        },
        None => Ok(()),
    };

    if let Err(err) = &result {
        crit!("{}", err);
    }

    Ok(())
}
