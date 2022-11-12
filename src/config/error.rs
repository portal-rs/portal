use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error while deserializing TOML")]
    Deserialize(#[from] toml::de::Error),

    #[error("Error while serializing TOML")]
    Serialize,

    #[error("Error while writing TOML config file")]
    Write,

    #[error("Error while reading TOML config file")]
    Read(#[from] std::io::Error),
}
