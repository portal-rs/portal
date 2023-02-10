use std::fmt::Display;

use binbuf::prelude::*;

use crate::types::dns::Name;

#[derive(Debug, Clone)]
pub struct MX {
    preference: u16,
    exchange: Name,
}

impl Display for MX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PREF: {} EX: {}", self.preference, self.exchange)
    }
}

impl Readable for MX {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let preference = u16::read::<E>(buf)?;
        let exchange = Name::read::<E>(buf)?;

        Ok(Self {
            preference,
            exchange,
        })
    }
}

impl Writeable for MX {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.preference.write::<E>(buf)?;
            self.exchange.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl MX {
    /// Returns the length of the [`MX`] record.
    pub fn len(&self) -> usize {
        // Returns the sum of EXCHANGE's len and 2 for PREFERENCE u16.
        self.exchange.len() + 2
    }
}
