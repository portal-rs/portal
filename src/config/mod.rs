use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml;

use crate::config::error::{ConfigError, ConfigErrorKind};

pub mod error;

// TODO (Techassi): Create custom 'partial' proc macro function
#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub collector: CollectorOptions,
    pub resolver: ResolverOptions,
    pub server: ServerOptions,
}

impl Config {
    /// Reads a TOML config file at `path` and returns a [`Config`] or any [`ConfigError`] encountered while reading
    /// and parsing. The function optionally validates the config.
    pub fn from_file(path: PathBuf, _validate: Option<bool>) -> Result<Self, ConfigError> {
        let b = match fs::read_to_string(path) {
            Ok(b) => b,
            Err(err) => {
                println!("{}", err);
                return Err(ConfigError::new(
                    ConfigErrorKind::Read,
                    "Failed to read config file",
                ));
            }
        };

        let c: Self = match toml::from_str(&b) {
            Ok(c) => c,
            Err(err) => {
                println!("{}", err);
                return Err(ConfigError::new(
                    ConfigErrorKind::Other,
                    "Failed to parse TOML",
                ));
            }
        };

        Ok(c)
    }
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            collector: Default::default(),
            resolver: Default::default(),
            server: Default::default(),
        };
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct CollectorOptions {
    pub max_entries: usize,
    pub anonymize: bool,
    pub interval: usize,
    pub enabled: bool,
    pub backend: String,
}

impl Default for CollectorOptions {
    fn default() -> Self {
        return Self {
            max_entries: 1000,
            anonymize: false,
            interval: 900,
            enabled: true,
            backend: String::from("default"),
        };
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ResolverOptions {
    pub cache_enabled: bool,
    pub upstream: String,
    pub max_expire: usize,
    pub hint_path: String,
    pub mode: String,
}

impl Default for ResolverOptions {
    fn default() -> Self {
        return Self {
            cache_enabled: true,
            upstream: String::from(""),
            max_expire: 300,
            hint_path: String::from(""),
            mode: String::from("r"),
        };
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ServerOptions {
    pub cache_enabled: bool,
    pub address: String,
    pub network: String,
}

impl Default for ServerOptions {
    fn default() -> Self {
        return Self {
            cache_enabled: true,
            address: String::from("127.0.0.1:53"),
            network: String::from("udp"),
        };
    }
}
