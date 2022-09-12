use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.7 (EXPERIMENTAL)
#[derive(Debug, Clone)]
pub struct MINFO {
    pub header: RRHeader,
    pub rmailbox: String,
    pub emailbox: String,
}

impl MINFO {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            rmailbox: String::new(),
            emailbox: String::new(),
        }
        .into();
    }
}

impl Record for MINFO {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        return (self.rmailbox.len() + self.emailbox.len() + 2) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for MINFO {
    fn to_string(&self) -> String {
        format!(
            "MINFO <RMAIL: {} - EMAIL: {}>",
            self.rmailbox, self.emailbox
        )
    }
}

impl PartialEq<Self> for MINFO {
    fn eq(&self, other: &Self) -> bool {
        self.rmailbox == other.rmailbox && self.emailbox == other.emailbox
    }
}
