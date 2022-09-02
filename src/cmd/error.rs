use std::fmt;

pub struct CmdError {
    message: String,
    cmd: String,
}

impl CmdError {
    pub fn new<T: Into<String>, S: Into<String>>(cmd: T, message: S) -> Self {
        return CmdError {
            message: message.into(),
            cmd: cmd.into(),
        };
    }
}

impl From<clap::Error> for CmdError {
    fn from(err: clap::Error) -> Self {
        return CmdError::new("root", err.to_string());
    }
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The '{}' command failed to run: {}",
            self.cmd, self.message
        )
    }
}

impl fmt::Debug for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CmdError")
            .field("message", &self.message)
            .field("command", &self.cmd)
            .finish()
    }
}
