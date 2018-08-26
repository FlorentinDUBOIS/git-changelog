//! # Build module
//!
//! The build module create rust files at build time
//! in order to inject some source code.
use askama;
use failure::{Error, ResultExt};
use git2::Repository;
use std::fs::File;
use std::io::Write;
use time::now_utc;

pub fn main() -> Result<(), Error> {
    // Transpile templates if they changed
    askama::rerun_if_templates_changed();

    // Load the current git repository and retrieve the last commit using the
    // HEAD current reference
    let repository = Repository::discover(".").context("Expect to have a git repository")?;
    let identifier = repository
        .revparse_single("HEAD")
        .context("Expect to have at least one git commit")?
        .id();

    // Retrieve the current time use UTC timezone
    let now = now_utc();

    // Generate the version file
    let mut file = File::create("src/version.rs")?;

    file.write(format!("const BUILD_DATE: &str = \"{}\";\n", now.rfc3339()).as_bytes())?;
    file.write(format!("const GITHASH: &str = \"{}\";\n", identifier).as_bytes())?;

    file.flush()?;
    file.sync_all()?;

    Ok(())
}
