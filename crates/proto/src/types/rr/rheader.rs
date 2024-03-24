use std::fmt::Display;

use binbuf::{
    read::{ReadBuffer, ReadError},
    write::{WriteBuffer, WriteError},
    Endianness, Readable, Writeable,
};
use serde::Serialize;
use snafu::{ResultExt, Snafu};

use crate::{
    constants::RECORD_FIXED_LENGTH,
    types::{
        dns::{Name, NameError},
        rr::{Class, RType},
    },
};

#[derive(Debug, Snafu)]
pub enum RHeaderError {
    #[snafu(display("failed to read NAME"))]
    ReadName { source: NameError },

    #[snafu(display("failed to write NAME"))]
    WriteName { source: NameError },

    #[snafu(display("failed to read TYPE"))]
    ReadType { source: ReadError },

    #[snafu(display("failed to write TYPE"))]
    WriteType { source: WriteError },

    #[snafu(display("failed to read CLASS"))]
    ReadClass { source: ReadError },

    #[snafu(display("failed to write CLASS"))]
    WriteClass { source: WriteError },

    #[snafu(display("failed to read TTL"))]
    ReadTTL { source: ReadError },

    #[snafu(display("failed to write TTL"))]
    WriteTTL { source: WriteError },

    #[snafu(display("failed to read RDLEN"))]
    ReadRdlen { source: ReadError },

    #[snafu(display("failed to write RDLEN"))]
    WriteRdlen { source: WriteError },
}

/// A resource record header (RHeader for short) is an abstraction over all
/// common fields of a resource record _except_ the RDATA field.
///
/// This abstraction does not exist in RFC 1034 and 1035. It is used in this
/// crate to simplify the implementation.
///
/// ### See
///
/// - <https://datatracker.ietf.org/doc/html/rfc1034#section-3.6>
/// - <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2>
#[derive(Debug, Clone, Default, Serialize)]
pub struct RHeader {
    name: Name,
    ty: RType,
    class: Class,
    ttl: u32,
    rdlen: u16,
}

impl Display for RHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N: {} T: {} C: {} TTL: {} ({})",
            self.name, self.ty, self.class, self.ttl, self.rdlen
        )
    }
}

impl Readable for RHeader {
    type Error = RHeaderError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let name = Name::read::<E>(buf).context(ReadNameSnafu)?;
        let ty = RType::read::<E>(buf).context(ReadTypeSnafu)?;
        let class = Class::read::<E>(buf).context(ReadClassSnafu)?;
        let ttl = u32::read::<E>(buf).context(ReadTTLSnafu)?;
        let rdlen = u16::read::<E>(buf).context(ReadRdlenSnafu)?;

        Ok(Self {
            name,
            ty,
            class,
            ttl,
            rdlen,
        })
    }
}

impl Writeable for RHeader {
    type Error = RHeaderError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        buf.enter();

        self.name.write::<E>(buf).context(WriteNameSnafu)?;
        self.ty.write::<E>(buf).context(WriteTypeSnafu)?;
        self.class.write::<E>(buf).context(WriteClassSnafu)?;
        self.ttl.write::<E>(buf).context(WriteTTLSnafu)?;
        self.rdlen.write::<E>(buf).context(WriteRdlenSnafu)?;

        Ok(buf.exit())
    }
}

impl RHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        RECORD_FIXED_LENGTH + self.name.size()
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name
    }

    pub fn ty(&self) -> &RType {
        &self.ty
    }

    pub fn set_ty(&mut self, ty: RType) {
        self.ty = ty
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class
    }

    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl
    }

    pub fn rdlen(&self) -> u16 {
        self.rdlen
    }

    pub fn set_rdlen(&mut self, rdlen: u16) {
        self.rdlen = rdlen
    }
}
