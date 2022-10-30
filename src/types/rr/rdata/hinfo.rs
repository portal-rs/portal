use crate::packing::{UnpackBuffer, UnpackBufferResult, Unpackable};

#[derive(Debug)]
pub struct HINFO {
    cpu: Vec<u8>,
    os: Vec<u8>,
}

impl Unpackable for HINFO {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let cpu = buf.unpack_character_string(255)?.to_vec();
        let os = buf.unpack_character_string(255)?.to_vec();

        Ok(Self { cpu, os })
    }
}
