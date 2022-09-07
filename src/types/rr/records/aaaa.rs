use std::net::Ipv6Addr;

use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc3596
#[derive(Debug)]
pub struct AAAA {
    pub header: RRHeader,
    pub address: Ipv6Addr,
}

impl AAAA {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            address: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
        }
        .into();
    }
}

impl Record for AAAA {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return 16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for AAAA {
    fn to_string(&self) -> String {
        format!("AAAA <{}>", self.address)
    }
}

impl PartialEq<Self> for AAAA {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
