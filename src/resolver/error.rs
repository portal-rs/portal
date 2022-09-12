use std::fmt;

pub struct ResolveError {
    message: String,
}

impl ResolveError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to resolve: {}", self.message)
    }
}

impl fmt::Debug for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolveError")
            .field("message", &self.message)
            .finish()
    }
}

pub struct ResolverError {
    message: String,
}

impl ResolverError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Resolver error: {}", self.message)
    }
}

impl fmt::Debug for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolverError")
            .field("message", &self.message)
            .finish()
    }
}
