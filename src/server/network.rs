use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub enum Network {
    Tcp,
    Udp,
}

pub struct NetworkError {
    input: String,
}

impl NetworkError {
    pub fn new<M: Into<String>>(input: M) -> Self {
        return Self {
            input: input.into(),
        };
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The input '{}' could not be parsed into a valid network",
            self.input
        )
    }
}

impl fmt::Debug for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetworkError")
            .field("input", &self.input)
            .finish()
    }
}

impl Network {
    pub fn parse<T: Into<String>>(input: T) -> Result<Self, NetworkError> {
        let input: String = input.into();
        match input.to_lowercase().as_str() {
            "tcp" | "tcp4" | "tcp6" => return Ok(Network::Tcp),
            "udp" | "udp4" | "udp6" => return Ok(Network::Udp),
            _ => return Err(NetworkError::new(input)),
        }
    }
}
