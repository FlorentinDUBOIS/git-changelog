//! # Build module
//!
//! The build module create rust files at build time
//! in order to inject some source code.
use std::{env, error::Error, fs::File, io::Write};

use chrono::Utc;
use git2::Repository;

pub fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Load the current git repository and retrieve the last commit using the
    // HEAD current reference
    let repository = Repository::discover(".")
        .map_err(|err| format!("Expect to have a git repository, {}", err))?;

    let identifier = repository
        .revparse_single("HEAD")
        .map_err(|err| format!("Expect to have at least one git commit, {}", err))?
        .id();

    let profile =
        env::var("PROFILE").map_err(|err| format!("Expect to be built using cargo, {}", err))?;

    // Generate the version file
    let mut file = File::create("src/version.rs")
        .map_err(|err| format!("could not create 'src/version.rs' file, {}", err))?;

    file.write(
        format!(
            "pub(crate) const BUILD_DATE: &str = \"{}\";\n",
            Utc::now().to_rfc3339(),
        )
        .as_bytes(),
    )?;
    file.write(format!("pub(crate) const GITHASH: &str = \"{}\";\n", identifier).as_bytes())?;
    file.write(format!("pub(crate) const PROFILE: &str = \"{}\";\n", profile).as_bytes())?;

    file.flush()?;
    file.sync_all()?;

    Ok(())
}
