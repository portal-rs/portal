use std::fmt::Display;

use crate::packing::{PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult};

#[derive(Debug, Clone)]
pub struct NULL {
    data: Vec<u8>,
}

impl Display for NULL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Packable for NULL {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        buf.pack_vec(&mut self.data.clone())
    }
}

impl NULL {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn unpack(buf: &mut UnpackBuffer, rdlen: u16) -> UnpackBufferResult<Self> {
        if rdlen > 0 {
            let data = buf.unpack_vec(rdlen as usize)?;
            return Ok(Self { data });
        }

        Ok(Self::new())
    }
}
