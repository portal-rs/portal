use std::fmt::Display;

use binbuf::prelude::*;

use crate::types::dns::Name;

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

impl Readable for SOA {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let mname = Name::read::<E>(buf)?;
        let rname = Name::read::<E>(buf)?;
        let serial = u32::read::<E>(buf)?;
        let refresh = u32::read::<E>(buf)?;
        let retry = u32::read::<E>(buf)?;
        let expire = u32::read::<E>(buf)?;
        let minimum = u32::read::<E>(buf)?;

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

impl Writeable for SOA {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.mname.write::<E>(buf)?;
            self.rname.write::<E>(buf)?;
            self.serial.write::<E>(buf)?;
            self.refresh.write::<E>(buf)?;
            self.retry.write::<E>(buf)?;
            self.expire.write::<E>(buf)?;
            self.minimum.write::<E>(buf)?
        };

        Ok(n)
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
