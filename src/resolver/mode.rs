use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Clone)]
pub enum ResolveMode {
    Recursive,
    Iterative,
    Forwarding,
}

#[derive(Debug, Error)]
pub struct ResolveModeError {
    input: String,
}

impl Display for ResolveModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid resolve mode {}", self.input)
    }
}

impl FromStr for ResolveMode {
    type Err = ResolveModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r" => Ok(Self::Recursive),
            "i" => Ok(Self::Iterative),
            "f" => Ok(Self::Forwarding),
            _ => Err(ResolveModeError { input: s.into() }),
        }
    }
}
