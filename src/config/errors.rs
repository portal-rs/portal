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
    pub fn new(kind: ConfigErrorKind, message: &str) -> Self {
        return ConfigError {
            message: message.to_string(),
            kind,
        };
    }
}
