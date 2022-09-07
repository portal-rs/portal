use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.4 (Obsolete)
#[derive(Debug)]
pub struct MD {
    pub header: RRHeader,
    pub madname: String,
}

impl MD {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            madname: String::new(),
        }
        .into();
    }
}

impl Record for MD {
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

impl ToString for MD {
    fn to_string(&self) -> String {
        format!("MD <{}>", self.madname)
    }
}

impl PartialEq<Self> for MD {
    fn eq(&self, other: &Self) -> bool {
        self.madname == other.madname
    }
}
