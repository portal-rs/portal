use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.3 (EXPERIMENTAL)
#[derive(Debug)]
pub struct MB {
    pub header: RRHeader,
    pub madname: String,
}

impl MB {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            madname: String::new(),
        }
        .into();
    }
}

impl Record for MB {
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

impl ToString for MB {
    fn to_string(&self) -> String {
        format!("MB <{}>", self.madname)
    }
}

impl PartialEq<Self> for MB {
    fn eq(&self, other: &Self) -> bool {
        self.madname == other.madname
    }
}
