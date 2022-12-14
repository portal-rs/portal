use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::dns::Name,
};

mod classes;
mod rdata;
mod rheader;
mod types;

pub use classes::*;
pub use rdata::*;
pub use rheader::*;
pub use types::*;

#[derive(Debug, Clone)]
pub struct Record {
    header: RHeader,
    data: RData,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            header: Default::default(),
            data: Default::default(),
        }
    }
}

impl ToString for Record {
    fn to_string(&self) -> String {
        format!(
            "HEADER: {}\nDATA: {}",
            self.header.to_string(),
            self.data.to_string()
        )
    }
}

impl Unpackable for Record {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let header = match RHeader::unpack(buf) {
            Ok(header) => header,
            Err(err) => return Err(err),
        };

        let data = match RData::unpack(buf, &header) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        Ok(Self { header, data })
    }
}

impl Packable for Record {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.header.pack(buf)?;
        self.data.pack(buf)
    }
}

impl Record {
    pub fn new() -> Self {
        Record::default()
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
        self.header.set_rdlen(self.len() as u16);
        self
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn is_edns(&self) -> bool {
        *self.header.ty() == Type::OPT
    }

    pub fn is_soa(&self) -> bool {
        *self.header.ty() == Type::SOA
    }
}
