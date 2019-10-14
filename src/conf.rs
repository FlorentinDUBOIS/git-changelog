//! # Config module
//!
//! The configuration module handle the changelog.toml file

use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use config::{Config, File};
use failure::{Error, ResultExt};
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
    pub kinds: HashMap<String, String>,
    pub repositories: Vec<Repository>,
}

impl TryFrom<PathBuf> for Configuration {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut conf = Config::default();

        conf.merge(File::from(path).required(true))
            .with_context(|err| format!("could not configure the file constraint, {}", err))?;

        ok!(conf.try_into::<Self>().with_context(|err| {
            format!("could not cast data structure into configuration, {}", err)
        })?)
    }
}
