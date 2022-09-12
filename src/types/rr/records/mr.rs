use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.8 (EXPERIMENTAL)
#[derive(Debug, Clone)]
pub struct MR {
    pub header: RRHeader,
    pub newname: String,
}

impl MR {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            newname: String::new(),
        }
        .into();
    }
}

impl Record for MR {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.newname.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for MR {
    fn to_string(&self) -> String {
        format!("MR <{}>", self.newname)
    }
}

impl PartialEq<Self> for MR {
    fn eq(&self, other: &Self) -> bool {
        self.newname == other.newname
    }
}
