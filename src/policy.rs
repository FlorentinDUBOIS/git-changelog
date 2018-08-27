//! # Policy module.
//!
//! The policy module manage the policy file.
use config::{Config, File};
use failure::Error;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
    pub scopes: Option<Vec<String>>,
    pub range: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Policy {
    pub kinds: HashMap<String, String>,
    pub repositories: Vec<Repository>,
}

impl Policy {
    pub fn from(path: &PathBuf) -> Result<Self, Error> {
        let mut policy = Config::default();

        policy.merge(File::from(path.to_owned()).required(true))?;

        Ok(policy.try_into::<Self>()?)
    }
}
