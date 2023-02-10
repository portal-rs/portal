use std::fmt::Display;

use binbuf::prelude::*;

#[derive(Debug, Clone)]
pub struct NULL {
    data: Vec<u8>,
}

impl Display for NULL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Writeable for NULL {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        Ok(buf.write(&mut self.data.clone()))
    }
}

impl NULL {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn read<E: Endianness>(buf: &mut ReadBuffer, rdlen: u16) -> Result<Self, BufferError> {
        if rdlen > 0 {
            let data = buf.read_vec(rdlen as usize)?;
            return Ok(Self { data });
        }

        Ok(Self::new())
    }
}
