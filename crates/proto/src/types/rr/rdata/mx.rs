use std::fmt::Display;

use binbuf::{Readable, Writeable};

use crate::types::dns::Name;

#[derive(Debug, Clone, Readable, Writeable)]
pub struct MX {
    preference: u16,
    exchange: Name,
}

impl Display for MX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PREF: {} EX: {}", self.preference, self.exchange)
    }
}

impl MX {
    /// Returns the size of the [`MX`] record.
    pub fn size(&self) -> usize {
        // Returns the sum of EXCHANGE's len and 2 for PREFERENCE u16.
        self.exchange.size() + 2
    }
}
