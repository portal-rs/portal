use std::fmt::Display;

use binbuf::{
    read::{ReadBuffer, ReadError, Readable},
    write::{WriteBuffer, WriteError, Writeable},
    Endianness,
};

use crate::constants;

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

impl Readable for HINFO {
    type Error = ReadError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let cpu = buf
            .read_char_string(Some(constants::MAX_CHAR_STRING_LENGTH))?
            .to_vec();
        let os = buf
            .read_char_string(Some(constants::MAX_CHAR_STRING_LENGTH))?
            .to_vec();

        Ok(Self { cpu, os })
    }
}

impl Writeable for HINFO {
    type Error = WriteError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        buf.write_char_string(self.cpu.as_slice(), Some(constants::MAX_CHAR_STRING_LENGTH))?;
        buf.write_char_string(self.os.as_slice(), Some(constants::MAX_CHAR_STRING_LENGTH))
    }
}

impl HINFO {
    pub fn size(&self) -> usize {
        self.cpu.len() + self.os.len()
    }
}
