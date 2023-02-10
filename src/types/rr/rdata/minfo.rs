use std::fmt::Display;

use binbuf::prelude::*;

use crate::types::dns::Name;

#[derive(Debug, Clone)]
pub struct MINFO {
    rmailbx: Name,
    emailbx: Name,
}

impl Display for MINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RMAILBOX: {} EMAILBOX: {}", self.rmailbx, self.emailbx)
    }
}

impl Readable for MINFO {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let rmailbx = Name::read::<E>(buf)?;
        let emailbx = Name::read::<E>(buf)?;

        Ok(Self { rmailbx, emailbx })
    }
}

impl Writeable for MINFO {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.rmailbx.write::<E>(buf)?;
            self.emailbx.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl MINFO {
    pub fn len(&self) -> usize {
        self.rmailbx.len() + self.emailbx.len()
    }
}
