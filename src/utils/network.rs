use std::{fmt::Display, str::FromStr};

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Clone, Copy)]
pub enum Network {
    Tcp,
    Udp,
}

#[derive(Debug, Error)]
pub struct NetworkError {
    input: String,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid network {}, expected tcp/udp", self.input)
    }
}

impl FromStr for Network {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tcp" | "tcp4" | "tcp6" => return Ok(Network::Tcp),
            "udp" | "udp4" | "udp6" => return Ok(Network::Udp),
            _ => Err(NetworkError { input: s.into() }),
        }
    }
}
