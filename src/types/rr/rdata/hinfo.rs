use std::fmt::Display;

use crate::{
    constants,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
};

#[derive(Debug, Clone)]
pub struct HINFO {
    cpu: Vec<u8>,
    os: Vec<u8>,
}

impl Display for HINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU: {:?} OS: {:?}", self.cpu, self.os)
    }
}

impl Unpackable for HINFO {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let cpu = buf
            .unpack_character_string(constants::dns::MAX_CHAR_STRING_LENGTH)?
            .to_vec();
        let os = buf
            .unpack_character_string(constants::dns::MAX_CHAR_STRING_LENGTH)?
            .to_vec();

        Ok(Self { cpu, os })
    }
}

impl Packable for HINFO {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        buf.pack_character_string(self.cpu.as_slice(), constants::dns::MAX_CHAR_STRING_LENGTH)?;
        buf.pack_character_string(self.os.as_slice(), constants::dns::MAX_CHAR_STRING_LENGTH)
    }
}

impl HINFO {
    pub fn len(&self) -> usize {
        self.cpu.len() + self.os.len()
    }
}
