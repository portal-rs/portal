use crate::{
    constants,
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
};

#[derive(Debug)]
pub struct HINFO {
    cpu: Vec<u8>,
    os: Vec<u8>,
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
