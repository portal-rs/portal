use std::fmt;

pub struct ClientError {
    message: String,
}

impl ClientError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientError")
            .field("message", &self.message)
            .finish()
    }
}
