mod error;
mod options;

pub use error::*;
pub use options::*;

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml;

pub struct Config {
    pub resolver: ResolverOptions,
    pub server: ServerOptions,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawConfig {
    pub collector: RawCollectorOptions,
    pub resolver: RawResolverOptions,
    pub server: RawServerOptions,
}

impl RawConfig {
    /// Reads a TOML config file at `path` and returns a [`RawConfig`] or any
    /// [`ConfigError`] encountered while reading and parsing. The function
    /// optionally validates the config.
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let b = fs::read_to_string(path)?;
        Ok(toml::from_str(&b)?)
    }

    /// Validates the [`RawConfig`] and if successful returns a validated
    /// [`Config`]. Returns [`ConfigError`] otherwise.
    pub fn validate(&self) -> Result<Config, ConfigError> {
        let resolver_opts = match self.resolver.validate() {
            Ok(opts) => opts,
            Err(err) => return Err(ConfigError::ResolverOptionError(err)),
        };

        let server_opts = match self.server.validate() {
            Ok(opts) => opts,
            Err(err) => return Err(ConfigError::ServerOptionError(err)),
        };

        Ok(Config {
            resolver: resolver_opts,
            server: server_opts,
        })
    }
}
