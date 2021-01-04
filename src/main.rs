//! # Changelog
//!
//! Generate a changelog using the git commit history
use std::{convert::TryFrom, error::Error, fs::File, io::Write, path::PathBuf, rc::Rc};

use slog_scope::{crit, debug, warn};
use structopt::StructOpt;

use crate::{
    conf::Configuration,
    parser::{Changelog, HTMLChangelog, MarkdownChangelog},
    version::{BUILD_DATE, GITHASH, PROFILE},
};

mod conf;
mod logger;
mod parser;
mod version;

#[derive(StructOpt, Clone, Debug)]
pub struct Args {
    /// Prints version information
    #[structopt(short = "V", long = "version", global = true)]
    pub version: bool,

    /// Check if the configuration is healthy
    #[structopt(short = "t", long = "check")]
    pub check: bool,

    /// Increase the log verbosity
    #[structopt(short = "v", global = true, parse(from_occurrences))]
    pub verbose: usize,

    /// Use the specified configuration file
    #[structopt(
        short = "c",
        long = "config",
        global = true,
        default_value = "changelog.toml"
    )]
    pub config: PathBuf,

    /// Output using the specified format (available formats are: html or markdown)
    #[structopt(short = "f", long = "format", default_value = "markdown")]
    pub format: String,

    /// Set the output destination
    #[structopt(short = "o", long = "output", default_value = "CHANGELOG")]
    pub output: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
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
        return Ok(());
    }

    let conf = match Configuration::try_from(args.config.to_owned()) {
        Ok(conf) => Rc::new(conf),
        Err(err) => {
            return Err(format!("could not load configuration, {}", err).into());
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
            return Err(format!("could not generate the changelog, {}", err).into());
        }
    };

    let (extension, content) = match args.format.as_str() {
        "html" => ("html", format!("{}", HTMLChangelog::from(changelog))),
        "markdown" => ("md", format!("{}", MarkdownChangelog::from(changelog))),
        format => {
            crit!("could not use the given value for formatting, the format '{}' is not yet implemented", format);
            return Err(format!("could not use the given value for formatting, the format '{}' is not yet implemented", format).into());
        }
    };

    let mut output = args.output;

    output.set_extension(extension);
    let mut file = File::create(output.to_owned())
        .map_err(|err| format!("could not create file '{:?}', {}", output, err))?;

    file.write_all(content.as_bytes())
        .map_err(|err| format!("could not write content, {}", err))?;
    file.flush()
        .map_err(|err| format!("could not flush content on disk, {}", err))?;
    file.sync_all()
        .map_err(|err| format!("could not sync content on disk, {}", err))?;

    Ok(())
}
