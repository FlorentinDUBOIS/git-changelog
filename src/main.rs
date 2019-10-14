//! # Changelog
//!
//! Generate a changelog using the git commit history

#[macro_use]
extern crate slog_scope;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use failure::Error;
use structopt::StructOpt;

use crate::conf::Configuration;
use crate::parser::Changelog;
use crate::version::{BUILD_DATE, GITHASH, PROFILE};

// library module should be declare first as it expose macros used by other modules
// https://doc.rust-lang.org/1.2.0/book/macros.html#scoping-and-macro-import/export
#[macro_use]
mod lib;
mod conf;
mod logger;
mod parser;
mod version;

#[derive(StructOpt, Clone, Debug)]
pub struct Args {
    /// Prints version information
    #[structopt(short = "V", long = "version")]
    pub version: bool,

    /// Check if the configuration is healthy
    #[structopt(short = "t", long = "check")]
    pub check: bool,

    /// Increase the log verbosity
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: usize,

    /// Use the specified configuration file
    #[structopt(short = "c", long = "config", default_value = "changelog.toml")]
    pub config: PathBuf,

    /// Set the output destination
    #[structopt(short = "o", long = "output", default_value = "CHANGELOG.md")]
    pub output: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<(), Error> {
    let _guard = logger::initialize(args.verbose);

    if PROFILE == "debug" {
        warn!("{} is running in \"debug\" mode", env!("CARGO_PKG_NAME"));
    }

    if args.version {
        let mut version = format!(
            "{} version {} {}\n",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            GITHASH
        );
        version += &format!("{} build date {}\n", env!("CARGO_PKG_NAME"), BUILD_DATE);
        version += &format!("{} profile {}\n", env!("CARGO_PKG_NAME"), PROFILE);

        println!("{}", version);
        return ok!();
    }

    let conf = match Configuration::try_from(args.config) {
        Ok(conf) => Rc::new(conf),
        Err(err) => {
            return err!("could not load configuration, {}", err);
        }
    };

    if args.check {
        debug!("{:?}", conf);
        println!("Configuration is healthy");
    }

    let changelog = match Changelog::try_from(conf) {
        Ok(changelog) => changelog,
        Err(err) => {
            crit!("could not generate the changelog"; "error" => err.to_string());
            return err!("could not generate the changelog, {}", err);
        }
    };

    let mut file = File::create(args.output)?;

    file.write_all(format!("{}\n", changelog).as_bytes())?;
    file.flush()?;
    file.sync_all()?;

    ok!()
}
