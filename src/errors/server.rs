use std::fmt::Display;

pub enum ServerError {
    InvalidResolverMode,
    InvalidAddress,
    InvalidNetwork,
    AlreadyRunning,
    BindFailure,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InvalidResolverMode => write!(f, "Invalid resolver mode (r/f/i)"),
            ServerError::InvalidAddress => write!(f, "Invalid bind address"),
            ServerError::InvalidNetwork => write!(f, "Invalid network (udp/tcp)"),
            ServerError::AlreadyRunning => write!(f, "Server is already running"),
            ServerError::BindFailure => write!(f, "Faild to bind socket"),
        }
    }
}
