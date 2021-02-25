use std::{convert::TryFrom, path::PathBuf};

use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: Option<String>,
    pub device_id: String,
    pub device_name: String,
    pub access_token: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct HomeServer {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct Sync {
    pub interval: u64,
}

#[derive(Deserialize, Clone)]
pub struct BotConfig {
    pub credentials: Credentials,
    pub server: HomeServer,
    pub sync: Sync,
    pub storage: Storage,
}
#[derive(Deserialize, Clone)]
pub struct Storage {
    pub path: String,
}

impl TryFrom<PathBuf> for BotConfig {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::from(path))?;

        s.try_into().map(Ok)?
    }
}
