use std::fmt::Display;

use crate::{
    constants,
    packing::{PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult},
};

#[derive(Debug, Clone)]
pub struct TXT {
    data: Vec<Vec<u8>>,
}

impl Display for TXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|v| format!("  {:?}", v))
                .collect::<String>()
        )
    }
}

impl Packable for TXT {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        for s in &self.data {
            buf.pack_character_string(s.as_slice(), constants::dns::MAX_CHAR_STRING_LENGTH)?;
        }

        Ok(())
    }
}

impl TXT {
    pub fn unpack(buf: &mut UnpackBuffer, rdlen: u16) -> UnpackBufferResult<Self> {
        let start_len = buf.len();
        let rdlen = rdlen as usize;
        let mut data = Vec::new();

        while start_len - buf.len() < rdlen {
            let char_string =
                buf.unpack_character_string(constants::dns::MAX_CHAR_STRING_LENGTH)?;
            data.push(char_string.to_vec());
        }

        Ok(TXT { data })
    }

    /// Returns the length of the [`TXT`] record.
    pub fn len(&self) -> usize {
        let mut len = 0;

        for v in &self.data {
            len += v.len()
        }

        len
    }
}
