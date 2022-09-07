use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.10
#[derive(Debug)]
pub struct NULL {
    pub header: RRHeader,
    pub data: String,
}

impl NULL {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            data: String::new(),
        }
        .into();
    }
}

impl Record for NULL {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return 0;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for NULL {
    fn to_string(&self) -> String {
        format!("NULL <{}>", self.data)
    }
}

impl PartialEq<Self> for NULL {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
