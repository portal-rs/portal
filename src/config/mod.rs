use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml;

use self::errors::{ConfigError, ConfigErrorKind};

mod errors;

#[derive(Deserialize)]
pub struct Config {
    collector: Option<CollectorOptions>,
}

#[derive(Deserialize)]
pub struct CollectorOptions {
    max_entries: Option<usize>,
    anonymize: Option<bool>,
    interval: Option<usize>,
    enabled: Option<bool>,
    backend: Option<String>,
}

pub fn default() -> Config {
    return Config {
        collector: Some(CollectorOptions {
            max_entries: Some(1000),
            anonymize: Some(false),
            interval: Some(900),
            enabled: Some(true),
            backend: Some(String::from("default")),
        }),
    };
}

/// Read reads a TOML config file at `path` and returns a [`Config`] or any error encountered while reading
pub fn read(path: PathBuf) -> Result<Config, ConfigError> {
    let b = match fs::read_to_string(path) {
        Ok(b) => b,
        Err(err) => {
            return Err(ConfigError::new(
                ConfigErrorKind::Read,
                "Failed to read config file",
            ))
        }
    };

    let c: Config = match toml::from_str(&b) {
        Ok(c) => c,
        Err(err) => {
            return Err(ConfigError::new(
                ConfigErrorKind::Other,
                "Failed to parse TOML",
            ))
        }
    };

    Ok(c)
}
