//! # Conf
//!
//! Configuration module parse `policy.toml` file
use config::{Config, File};
use failure::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repositories {
  pub paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
  pub repositories: Repositories,
}

impl Policy {
  fn new() -> Result<Config, Error> {
    let mut policy = Config::default();

    policy.set_default("repositories.paths", vec!["."])?;

    Ok(policy)
  }

  pub fn from(path: &PathBuf) -> Result<Self, Error> {
    let mut policy = Self::new()?;

    policy.merge(File::from(path.to_owned()))?;

    Ok(policy.try_into::<Self>()?)
  }
}
