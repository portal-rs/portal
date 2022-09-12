use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.2
#[derive(Debug, Clone)]
pub struct HINFO {
    pub header: RRHeader,
    pub cpu: String,
    pub os: String,
}

impl HINFO {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            cpu: String::new(),
            os: String::new(),
        }
        .into();
    }
}

impl Record for HINFO {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.cpu.len() + self.os.len() + 2) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for HINFO {
    fn to_string(&self) -> String {
        format!("HINFO <CPU: {} - OS: {}>", self.cpu, self.os)
    }
}

impl PartialEq<Self> for HINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cpu == other.cpu && self.os == other.os
    }
}
