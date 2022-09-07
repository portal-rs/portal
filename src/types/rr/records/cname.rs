use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.1
#[derive(Debug)]
pub struct CNAME {
    pub header: RRHeader,
    pub target: String,
}

impl CNAME {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            target: String::new(),
        }
        .into();
    }
}

impl Record for CNAME {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.target.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for CNAME {
    fn to_string(&self) -> String {
        format!("CNAME <{}>", self.target)
    }
}

impl PartialEq<Self> for CNAME {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}
