use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml;

use self::errors::{ConfigError, ConfigErrorKind};

pub mod errors;

#[derive(Deserialize)]
pub struct Config {
    collector: Option<CollectorOptions>,
    resolver: Option<ResolverOptions>,
}

#[derive(Deserialize)]
pub struct CollectorOptions {
    max_entries: Option<usize>,
    anonymize: Option<bool>,
    interval: Option<usize>,
    enabled: Option<bool>,
    backend: Option<String>,
}

#[derive(Deserialize)]
pub struct ResolverOptions {
    cache_enabled: Option<bool>,
    upstream: Option<String>,
    max_expire: Option<usize>,
    hint_path: Option<String>,
    mode: Option<String>,
}

impl Config {
    pub fn validate(&self) -> Result<&Self, ConfigError> {
        Ok(self)
    }
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
        resolver: Some(ResolverOptions {
            cache_enabled: Some(true),
            upstream: Some(String::from("")),
            max_expire: Some(300),
            hint_path: Some(String::from("")),
            mode: Some(String::from("r")),
        }),
    };
}

/// Read reads a TOML config file at `path` and returns a [`Config`] or any [`ConfigError`] encountered while reading
/// and parsing. The function optionally validates the config.
pub fn read(path: PathBuf, validate: Option<bool>) -> Result<Config, ConfigError> {
    let b = match fs::read_to_string(path) {
        Ok(b) => b,
        Err(_) => {
            return Err(ConfigError::new(
                ConfigErrorKind::Read,
                "Failed to read config file",
            ))
        }
    };

    let c: Config = match toml::from_str(&b) {
        Ok(c) => c,
        Err(_) => {
            return Err(ConfigError::new(
                ConfigErrorKind::Other,
                "Failed to parse TOML",
            ))
        }
    };

    if validate.unwrap_or(false) {
        match c.validate() {
            Err(err) => return Err(err),
            Ok(_) => {}
        };
    }

    Ok(c)
}
