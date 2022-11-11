use std::fmt::Display;

mod client;
mod config;
mod proto;
mod server;

pub use client::*;
pub use config::*;
pub use proto::*;
pub use server::*;

pub struct AppError {
    variant: AppErrorVariant,
    details: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.variant, self.details)
    }
}

impl AppError {
    pub fn new<D>(details: D, variant: AppErrorVariant) -> Self
    where
        D: Into<String>,
    {
        Self {
            variant: variant,
            details: details.into(),
        }
    }
}

pub enum AppErrorVariant {
    ProtocolError(ProtocolError),
    ConfigError(ConfigError),
    ServerError(ServerError),
    ClientError(ClientError),
    Generic(String),
}

impl Display for AppErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErrorVariant::ProtocolError(err) => write!(f, "Protocol error: {}", err),
            AppErrorVariant::ConfigError(err) => write!(f, "Config error: {}", err),
            AppErrorVariant::ServerError(err) => write!(f, "Server error: {}", err),
            AppErrorVariant::ClientError(err) => write!(f, "Client error: {}", err),
            AppErrorVariant::Generic(err) => write!(f, "Error: {}", err),
        }
    }
}
