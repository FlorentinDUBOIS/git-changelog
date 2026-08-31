//! # Config module
//!
//! The configuration module handle the changelog.toml file

use std::{collections::BTreeMap, convert::TryFrom, error::Error, path::PathBuf};

use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
    pub scopes: Option<Vec<String>>,
    pub range: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub kinds: BTreeMap<String, String>,
    pub repositories: Vec<Repository>,
}

impl TryFrom<PathBuf> for Configuration {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Config::builder()
            .add_source(File::from(path).required(true))
            .build()?
            .try_deserialize()?)
    }
}
