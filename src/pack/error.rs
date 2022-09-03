use std::fmt;

pub struct UnpackError {
    message: String,
}

impl UnpackError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnpackError")
            .field("message", &self.message)
            .finish()
    }
}
