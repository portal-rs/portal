use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.5 (Obsolete)
#[derive(Debug)]
pub struct MF {
    pub header: RRHeader,
    pub madname: String,
}

impl MF {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            madname: String::new(),
        }
        .into();
    }
}

impl Record for MF {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.madname.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for MF {
    fn to_string(&self) -> String {
        format!("MF <{}>", self.madname)
    }
}

impl PartialEq<Self> for MF {
    fn eq(&self, other: &Self) -> bool {
        self.madname == other.madname
    }
}
