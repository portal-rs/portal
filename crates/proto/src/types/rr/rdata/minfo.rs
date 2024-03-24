use std::fmt::Display;

use binbuf::{Readable, Writeable};

use crate::types::dns::Name;

#[derive(Debug, Clone, Readable, Writeable)]
pub struct MINFO {
    rmailbx: Name,
    emailbx: Name,
}

impl Display for MINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RMAILBOX: {} EMAILBOX: {}", self.rmailbx, self.emailbx)
    }
}

impl MINFO {
    pub fn size(&self) -> usize {
        self.rmailbx.size() + self.emailbx.size()
    }
}
