use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.6 (EXPERIMENTAL)
#[derive(Debug)]
pub struct MG {
    pub header: RRHeader,
    pub mgmname: String,
}

impl MG {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            mgmname: String::new(),
        }
        .into();
    }
}

impl Record for MG {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.mgmname.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for MG {
    fn to_string(&self) -> String {
        format!("MG <{}>", self.mgmname)
    }
}

impl PartialEq<Self> for MG {
    fn eq(&self, other: &Self) -> bool {
        self.mgmname == other.mgmname
    }
}
