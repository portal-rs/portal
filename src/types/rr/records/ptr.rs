use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.12
#[derive(Debug, Clone)]
pub struct PTR {
    pub header: RRHeader,
    pub ptrdname: String,
}

impl PTR {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            ptrdname: String::new(),
        }
        .into();
    }
}

impl Record for PTR {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.ptrdname.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for PTR {
    fn to_string(&self) -> String {
        format!("PTR <{}>", self.ptrdname)
    }
}

impl PartialEq<Self> for PTR {
    fn eq(&self, other: &Self) -> bool {
        self.ptrdname == other.ptrdname
    }
}
