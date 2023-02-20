use std::fmt::Display;

use binbuf::prelude::*;

use crate::constants;

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
                .map(|v| format!("  {v:?}"))
                .collect::<String>()
        )
    }
}

impl Writeable for TXT {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = 0;

        for s in &self.data {
            n +=
                buf.write_char_string(s.as_slice(), Some(constants::dns::MAX_CHAR_STRING_LENGTH))?;
        }

        Ok(n)
    }
}

impl TXT {
    pub fn read<E: Endianness>(buf: &mut ReadBuffer, rdlen: u16) -> Result<Self, BufferError> {
        let start_len = buf.len();
        let rdlen = rdlen as usize;
        let mut data = Vec::new();

        while start_len - buf.len() < rdlen {
            let char_string = buf.read_char_string(Some(constants::dns::MAX_CHAR_STRING_LENGTH))?;
            data.push(char_string.to_vec());
        }

        Ok(TXT { data })
    }

    /// Returns the length of the [`TXT`] record.
    pub fn size(&self) -> usize {
        let mut len = 0;

        for v in &self.data {
            len += v.len()
        }

        len
    }
}
