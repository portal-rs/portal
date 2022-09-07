use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.14
#[derive(Debug)]
pub struct TXT {
    pub header: RRHeader,
    pub data: String,
}

impl TXT {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            data: String::new(),
        }
        .into();
    }
}

impl Record for TXT {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.data.len() + 1) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for TXT {
    fn to_string(&self) -> String {
        format!("TXT <{}>", self.data)
    }
}

impl PartialEq<Self> for TXT {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
