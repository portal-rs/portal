use std::fmt::Display;

pub enum ConfigError {
    Deserialize,
    Serialize,
    Write,
    Read,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Deserialize => write!(f, "Error while deserializing TOML"),
            ConfigError::Serialize => write!(f, "Error while serializing TOML"),
            ConfigError::Write => write!(f, "Error while writing TOML config file"),
            ConfigError::Read => write!(f, "Error while reading TOML config file"),
        }
    }
}
