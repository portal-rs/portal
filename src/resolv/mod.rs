//! Module to parse resolv.conf files

use std::{
    fs,
    net::{AddrParseError, IpAddr},
    path::PathBuf,
    str::FromStr,
};

use thiserror::Error;

enum ResolvParseState<'a> {
    Newline,
    Comment,
    Option(&'a str),
}

impl<'a> Default for ResolvParseState<'a> {
    fn default() -> Self {
        Self::Newline
    }
}

#[derive(Debug, Error)]
pub enum ResolvParseError {
    #[error("Invalid key-value pair")]
    InvalidKeyValuePair,

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Invalid value")]
    InvalidValue,

    #[error("Failed to parse IP address: {0}")]
    AddrParseError(#[from] AddrParseError),
}

#[derive(Debug, Error)]
pub enum ResolvConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ResolvParseError(#[from] ResolvParseError),
}

#[derive(Debug, Default)]
pub struct ResolvConfig(Vec<ResolvOption>);

impl FromStr for ResolvConfig {
    type Err = ResolvParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = ResolvConfig::default();

        let mut state = ResolvParseState::default();
        let mut lines = s.lines();

        loop {
            state = match state {
                ResolvParseState::Newline => match lines.next() {
                    Some(line) => {
                        if line.is_empty() {
                            continue;
                        }

                        if line.starts_with(['#', ';']) {
                            state = ResolvParseState::Comment;
                            continue;
                        }

                        ResolvParseState::Option(line)
                    }
                    None => break, // EOF
                },
                ResolvParseState::Comment => {
                    // Don't keep track of comments
                    ResolvParseState::Newline
                }
                ResolvParseState::Option(line) => {
                    let parts = match line.split_once(' ') {
                        Some(parts) => parts,
                        None => return Err(ResolvParseError::InvalidKeyValuePair),
                    };

                    let option = ResolvOption::try_from(parts)?;
                    config.0.push(option);

                    ResolvParseState::Newline
                }
            }
        }

        Ok(config)
    }
}

impl ResolvConfig {
    pub fn from_file(path: PathBuf) -> Result<Self, ResolvConfigError> {
        let b = fs::read_to_string(path)?;
        Ok(b.parse()?)
    }

    pub fn options(&self) -> &Vec<ResolvOption> {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum ResolvOption {
    Nameserver(IpAddr),
    Search(Vec<String>),
    Sortlist(Vec<(IpAddr, Option<IpAddr>)>),
    Options(Vec<String>),
}

impl TryFrom<(&str, &str)> for ResolvOption {
    type Error = ResolvParseError;

    fn try_from(input: (&str, &str)) -> Result<Self, Self::Error> {
        let (key, value) = input;

        match key {
            "nameserver" => {
                // TODO (Techassi): We should not throw away the IPv6 zone identifier, but instead use it to bind to the
                // correct network interface.
                let value = if value.contains([':', '%']) {
                    value.split('%').nth(0).unwrap_or(value)
                } else {
                    value
                };
                let ip_addr = value.parse::<IpAddr>()?;
                Ok(Self::Nameserver(ip_addr))
            }
            "search" => {
                let names: Vec<String> = value.split(' ').map(|n| n.to_string()).collect();
                Ok(Self::Search(names))
            }
            "sortlist" => {
                let mut pairs = Vec::new();

                // Split each potential ip/mask by space
                for maybe_pair in value.split(' ') {
                    // Split by / - If this returns None, the mask was not provided
                    let pair = match maybe_pair.split_once('/') {
                        Some((addr, mask)) => {
                            let addr = addr.parse::<IpAddr>()?;
                            let mask = mask.parse::<IpAddr>()?;

                            (addr, Some(mask))
                        }
                        None => {
                            let addr = maybe_pair.parse::<IpAddr>()?;

                            (addr, None)
                        }
                    };

                    pairs.push(pair);
                }

                Ok(Self::Sortlist(pairs))
            }
            "options" => {
                let options: Vec<String> = value.split(' ').map(|n| n.to_string()).collect();
                Ok(Self::Options(options))
            }
            _ => Err(ResolvParseError::InvalidKey(key.to_string())),
        }
    }
}
