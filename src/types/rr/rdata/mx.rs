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
