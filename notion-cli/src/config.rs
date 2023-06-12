//! Configuration for the CLI.

use std::{
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
};

use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Config values.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// API token for the Notion API.
    pub api_token: String,
}

/// Get the config file path from the config directory.
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = home_dir()
        .context(Error::Config)?
        .join(".config")
        .join("notion-cli");

    if !config_dir.exists() {
        create_dir_all(config_dir.clone())?;
    }

    let config_file = PathBuf::new().join(config_dir).join("config.toml");

    if !config_file.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_file.clone())?;
    }

    Ok(config_file)
}

/// Load config from the config file
pub fn load_config() -> Result<Config> {
    let config_file = get_config_path()?;

    let config = config::Config::builder()
        .add_source(config::File::from(config_file.clone()))
        .add_source(config::Environment::with_prefix("NOTION"))
        .build()?
        .try_deserialize::<Config>()
        .context(Error::IncompleteConfig(config_file.display().to_string()))?;

    Ok(config)
}
