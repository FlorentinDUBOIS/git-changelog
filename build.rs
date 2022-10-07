//! # Build module
//!
//! The build module create rust files at build time
//! in order to inject some source code.
use std::{
    env,
    error::Error,
    fs::File,
    io::{Read, Write},
};

use chrono::Utc;
use git2::Repository;

const TEMPLATE_SRC: &str = "templates/changelog.mjml";
const TEMPLATE_DST: &str = "templates/changelog.html";

fn build_source_code_info() -> Result<(), Box<dyn Error + Send + Sync>> {
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
    )
    .map_err(|err| {
        format!(
            "could not write build date in file 'src/version.rs', {}",
            err
        )
    })?;
    file.write(format!("pub(crate) const GITHASH: &str = \"{}\";\n", identifier).as_bytes())
        .map_err(|err| format!("could not write githash in file 'src/version.rs', {}", err))?;
    file.write(format!("pub(crate) const PROFILE: &str = \"{}\";\n", profile).as_bytes())
        .map_err(|err| format!("could not write profile in file 'src/version.rs', {}", err))?;

    file.flush()
        .map_err(|err| format!("could not flush file 'src/version.rs', {}", err))?;
    file.sync_all()
        .map_err(|err| format!("could not sync file 'src/version.rs', {}", err))?;

    Ok(())
}

fn build_mjml_template() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut src_file = File::open(TEMPLATE_SRC)?;
    let mut dst_file = File::create(TEMPLATE_DST)?;
    let mut input_buffer = String::new();
    src_file.read_to_string(&mut input_buffer)?;
    let root = mrml::parse(input_buffer).map_err(|_| "could not parse input file")?;
    let opts = mrml::prelude::render::Options::default();
    let content = root
        .render(&opts)
        .map_err(|_| "could not render mjml template")?;
    dst_file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    build_source_code_info()?;
    build_mjml_template()?;
    Ok(())
}
