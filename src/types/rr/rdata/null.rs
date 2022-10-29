use crate::packing::{UnpackBuffer, UnpackBufferResult};

#[derive(Debug)]
pub struct NULL {
    data: Vec<u8>,
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
