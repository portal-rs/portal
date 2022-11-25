use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::dns::Name,
};

#[derive(Debug)]
pub struct MX {
    preference: u16,
    exchange: Name,
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

impl MX {
    /// Returns the length of the [`MX`] record.
    pub fn len(&self) -> usize {
        // Returns the sum of EXCHANGE's len and 2 for PREFERENCE u16.
        return self.exchange.len() + 2;
    }
}
