use std::fmt::Display;

use binbuf::{
    macros::bytes_written, read::ReadBuffer, write::WriteBuffer, Endianness, Readable, Writeable,
};
use serde::{ser::SerializeStruct, Serialize};
use snafu::{ResultExt, Snafu};

use crate::types::dns::Name;

mod classes;
mod rdata;
mod rheader;
mod types;

pub use classes::*;
pub use rdata::*;
pub use rheader::*;
pub use types::*;

#[derive(Debug, Snafu)]
pub enum RecordError {
    #[snafu(display("failed to read RHEADER"))]
    ReadHeader { source: RHeaderError },

    #[snafu(display("failed to write RHEADER"))]
    WriteHeader { source: RHeaderError },

    #[snafu(display("failed to read RDATA"))]
    ReadData { source: RDataError },

    #[snafu(display("failed to write RDATA"))]
    WriteData { source: RDataError },
}

/// ### Resource Records Definition (RFC 1034)
///
/// A domain name identifies a node. Each node has a set of resource
/// information, which may be empty. The set of resource information
/// associated with a particular name is composed of separate resource
/// records (RRs). The order of RRs in a set is not significant, and need
/// not be preserved by name servers, resolvers, or other parts of the DNS.
///
/// ### See
///
/// - <https://datatracker.ietf.org/doc/html/rfc1034#section-3.6>
/// - <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2>
#[derive(Debug, Clone, Default)]
pub struct Record {
    header: RHeader,
    data: RData,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.header.name(),
            self.header.ttl(),
            self.header.class(),
            self.header.ty(),
            self.rdata()
        )
    }
}

impl Readable for Record {
    type Error = RecordError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let header = RHeader::read::<E>(buf).context(ReadHeaderSnafu)?;
        let data = RData::read::<E>(buf, &header).context(ReadDataSnafu)?;

        Ok(Self { header, data })
    }
}

impl Writeable for Record {
    type Error = RecordError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.header.write::<E>(buf).context(WriteHeaderSnafu)?;
            self.data.write::<E>(buf).context(WriteDataSnafu)?
        };

        Ok(n)
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Record", 2)?;
        state.serialize_field("header", &self.header)?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

impl Record {
    pub fn new() -> Self {
        Record::default()
    }

    pub fn new_with_header(header: RHeader) -> Self {
        Self {
            header,
            ..Default::default()
        }
    }

    /// Set the complete resource record header [`RHeader`] at once.
    pub fn set_header(&mut self, header: RHeader) -> &mut Self {
        self.header = header;
        self
    }

    /// Set the resource record header domain name.
    pub fn set_header_name(&mut self, name: Name) -> &mut Self {
        self.header.set_name(name);
        self
    }

    pub fn header(&self) -> &RHeader {
        &self.header
    }

    /// Set the [`RData`] section of the [`Record`].
    pub fn set_rdata(&mut self, rdata: RData) -> &mut Self {
        self.data = rdata;
        self
    }

    pub fn rdata(&self) -> &RData {
        &self.data
    }

    pub fn normalize_rdlen(&mut self) -> &mut Self {
        self.header.set_rdlen(self.size() as u16);
        self
    }

    pub fn size(&self) -> usize {
        self.data.size()
    }

    pub fn is_edns(&self) -> bool {
        *self.header.ty() == RType::OPT
    }

    pub fn is_soa(&self) -> bool {
        *self.header.ty() == RType::SOA
    }
}
