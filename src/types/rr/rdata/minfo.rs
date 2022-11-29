use std::fmt::Display;

use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::dns::Name,
};

#[derive(Debug)]
pub struct MINFO {
    rmailbx: Name,
    emailbx: Name,
}

impl Display for MINFO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RMAILBOX: {} EMAILBOX: {}", self.rmailbx, self.emailbx)
    }
}

impl Unpackable for MINFO {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let rmailbx = Name::unpack(buf)?;
        let emailbx = Name::unpack(buf)?;

        Ok(Self { rmailbx, emailbx })
    }
}

impl Packable for MINFO {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.rmailbx.pack(buf)?;
        self.emailbx.pack(buf)
    }
}

impl MINFO {
    pub fn len(&self) -> usize {
        return self.rmailbx.len() + self.emailbx.len();
    }
}
