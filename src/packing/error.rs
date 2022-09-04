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
        write!(f, "Failed to unpack: {}", self.message)
    }
}

impl fmt::Debug for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnpackError")
            .field("message", &self.message)
            .finish()
    }
}

pub struct PackError {
    message: String,
}

impl PackError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to pack: {}", self.message)
    }
}

impl fmt::Debug for PackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackError")
            .field("message", &self.message)
            .finish()
    }
}
