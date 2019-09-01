//! # Changelog
//!
//! Generate a changelog using the git commit history
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[macro_use]
extern crate structopt_derive;

use failure::Error;

use crate::cmd::{Args, Command};

pub mod cmd;
pub mod logger;
pub mod parser;
pub mod policy;

#[paw::main]
pub fn main(args: Args) -> Result<(), Error> {
    let _guard = logger::initialize(args.verbose)?;

    let result = match args.cmd {
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
