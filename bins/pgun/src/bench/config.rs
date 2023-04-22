use std::{
    fs,
    net::{AddrParseError, SocketAddr},
    path::PathBuf,
};

use portal_proto::{Name, NameError, RType, RTypeParseError};
use serde::Deserialize;
use thiserror::Error;
use toml;

#[derive(Debug, Error)]
pub enum BenchConfigError {
    #[error("Read IO error: {0}")]
    Read(#[from] std::io::Error),

    #[error("TOML deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("Name parsing error: {0}")]
    NameError(#[from] NameError),

    #[error("Type parse error: {0}")]
    TypeParseError(#[from] RTypeParseError),

    #[error("Socket address parse error: {0}")]
    AddrParseError(#[from] AddrParseError),
}

#[derive(Debug, Deserialize)]
pub struct RawBenchConfig {
    bench: RawBenchOptions,
    data: RawDataOptions,
    server: String,
}

#[derive(Debug)]
pub struct BenchConfig {
    pub bench: BenchOptions,
    pub data: DataOptions,
    pub server: SocketAddr,
}

#[derive(Debug, Deserialize)]
pub struct RawBenchOptions {
    delay: usize,
    runs: usize,
}

#[derive(Debug)]
pub struct BenchOptions {
    pub delay: usize,
    pub runs: usize,
}

#[derive(Debug, Deserialize)]
pub struct RawDataOptions {
    domains: Vec<String>,
    types: Vec<String>,
}

#[derive(Debug)]
pub struct DataOptions {
    pub domains: Vec<Name>,
    pub types: Vec<RType>,
}

impl TryFrom<RawBenchConfig> for BenchConfig {
    type Error = BenchConfigError;

    fn try_from(value: RawBenchConfig) -> Result<Self, Self::Error> {
        let domains = value
            .data
            .domains
            .iter()
            .map(|d| Name::try_from(d.clone()))
            .collect::<Result<Vec<Name>, _>>()?;

        let types = value
            .data
            .types
            .iter()
            .map(|t| RType::try_from(t.clone()))
            .collect::<Result<Vec<RType>, _>>()?;

        let server = value.server.parse::<SocketAddr>()?;

        Ok(Self {
            bench: BenchOptions {
                delay: value.bench.delay,
                runs: value.bench.runs,
            },
            data: DataOptions { domains, types },
            server,
        })
    }
}

impl BenchConfig {
    pub fn from_file(path: PathBuf) -> Result<Self, BenchConfigError> {
        let b = fs::read_to_string(path)?;
        let c: RawBenchConfig = toml::from_str(&b)?;

        Self::try_from(c)
    }
}
