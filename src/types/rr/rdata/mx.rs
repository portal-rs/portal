use std::fmt::Display;

use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::dns::Name,
};

#[derive(Debug)]
pub struct MX {
    preference: u16,
    exchange: Name,
}

impl Display for MX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PREF: {} EX: {}", self.preference, self.exchange)
    }
}

impl Unpackable for MX {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let preference = u16::unpack(buf)?;
        let exchange = Name::unpack(buf)?;

        Ok(Self {
            preference,
            exchange,
        })
    }
}

impl Packable for MX {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.preference.pack(buf)?;
        self.exchange.pack(buf)
    }
}

impl MX {
    /// Returns the length of the [`MX`] record.
    pub fn len(&self) -> usize {
        // Returns the sum of EXCHANGE's len and 2 for PREFERENCE u16.
        return self.exchange.len() + 2;
    }
}
