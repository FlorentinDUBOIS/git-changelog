//! # Version module
//!
//! The version module provide the version command.
use failure::Error;

include!("../version.rs");

/// ## Version command
///
/// Display the version on the standard output.
pub fn version() -> Result<(), Error> {
  let name = env!("CARGO_PKG_NAME");
  let semver = env!("CARGO_PKG_VERSION");

  println!("{} {} {}", name, semver, GITHASH);
  println!("{} build date {}", name, BUILD_DATE);
  Ok(())
}
