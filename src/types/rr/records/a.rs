use std::net::Ipv4Addr;

use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.4.1
#[derive(Debug)]
pub struct A {
    pub header: RRHeader,
    pub address: Ipv4Addr,
}

impl A {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            address: Ipv4Addr::new(0, 0, 0, 0),
        }
        .into();
    }
}

impl Record for A {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return 4;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for A {
    fn to_string(&self) -> String {
        format!("A <{}>", self.address)
    }
}

impl PartialEq<Self> for A {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
