use crate::{
    constants,
    packing::{UnpackBuffer, UnpackBufferResult},
};

#[derive(Debug)]
pub struct TXT {
    data: Vec<Vec<u8>>,
}

impl TXT {
    pub fn unpack(buf: &mut UnpackBuffer, rdlen: u16) -> UnpackBufferResult<Self> {
        let start_len = buf.rest_len();
        let rdlen = rdlen as usize;
        let mut data = Vec::new();

        while start_len - buf.rest_len() < rdlen {
            let char_string =
                buf.unpack_character_string(constants::dns::MAX_CHAR_STRING_LENGTH)?;
            data.push(char_string.to_vec());
        }

        Ok(TXT { data })
    }
}
