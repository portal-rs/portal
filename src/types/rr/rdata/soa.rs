use std::fmt::Display;

use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::dns::Name,
};

#[derive(Debug, Clone)]
pub struct SOA {
    mname: Name,
    rname: Name,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

impl Unpackable for SOA {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let mname = Name::unpack(buf)?;
        let rname = Name::unpack(buf)?;
        let serial = u32::unpack(buf)?;
        let refresh = u32::unpack(buf)?;
        let retry = u32::unpack(buf)?;
        let expire = u32::unpack(buf)?;
        let minimum = u32::unpack(buf)?;

        Ok(Self {
            mname,
            rname,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        })
    }
}

impl Packable for SOA {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.mname.pack(buf)?;
        self.rname.pack(buf)?;
        self.serial.pack(buf)?;
        self.refresh.pack(buf)?;
        self.retry.pack(buf)?;
        self.expire.pack(buf)?;
        self.minimum.pack(buf)
    }
}

impl Display for SOA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SOA {} {} (\n  {}; serial\n  {}; refresh\n  {}; retry\n  {}; expire\n  {}; minimum\n)",
            self.mname,
            self.rname,
            self.serial,
            self.refresh,
            self.retry,
            self.expire,
            self.minimum
        )
    }
}

impl SOA {
    /// Returns the length of the [`SOA`] record.
    pub fn len(&self) -> usize {
        // Returns the sum of MNAME's len, RNAME's len and a fixed length. The
        // fixed part is 5 x 4 octets for five u32.
        self.mname.len() + self.rname.len() + 20
    }

    pub fn get_mname(&self) -> &Name {
        &self.mname
    }
}
