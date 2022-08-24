use std::fmt;

pub struct ConfigError {
    kind: ConfigErrorKind,
    message: String,
}

pub enum ConfigErrorKind {
    Read,
    Write,
    Other,
}

impl ConfigError {
    pub fn new<T: Into<String>>(kind: ConfigErrorKind, message: T) -> Self {
        return ConfigError {
            message: message.into(),
            kind,
        };
    }

    pub fn to_string(self) -> String {
        return format!("Config error ({}): {}", self.kind, self.message);
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ConfigErrorKind {
    pub fn to_string(self) -> String {
        match self {
            ConfigErrorKind::Read => String::from("read"),
            ConfigErrorKind::Write => String::from("write"),
            ConfigErrorKind::Other => String::from("other"),
        }
    }
}

impl fmt::Display for ConfigErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
