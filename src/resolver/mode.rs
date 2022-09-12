use std::fmt;

#[derive(Clone)]
pub enum ResolveMode {
    Recursive,
    Iterative,
    Forwarding,
}

pub struct ResolveModeError {
    input: String,
}

impl ResolveModeError {
    pub fn new<M: Into<String>>(input: M) -> Self {
        return Self {
            input: input.into(),
        };
    }
}

impl fmt::Display for ResolveModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid resolve mode: {}", self.input)
    }
}

impl fmt::Debug for ResolveModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolveModeError")
            .field("input", &self.input)
            .finish()
    }
}

impl ResolveMode {
    pub fn parse<T: Into<String>>(input: T) -> Result<Self, ResolveModeError> {
        let mode: String = input.into();
        match mode.to_lowercase().as_str() {
            "r" => Ok(Self::Recursive),
            "i" => Ok(Self::Iterative),
            "f" => Ok(Self::Forwarding),
            _ => Err(ResolveModeError::new(mode)),
        }
    }
}
