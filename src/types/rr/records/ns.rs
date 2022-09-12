use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.11
#[derive(Debug, Clone)]
pub struct NS {
    pub header: RRHeader,
    pub nsdname: String,
}

impl NS {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            nsdname: String::new(),
        }
        .into();
    }
}

impl Record for NS {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.nsdname.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for NS {
    fn to_string(&self) -> String {
        format!("NS <{}>", self.nsdname)
    }
}

impl PartialEq<Self> for NS {
    fn eq(&self, other: &Self) -> bool {
        self.nsdname == other.nsdname
    }
}
