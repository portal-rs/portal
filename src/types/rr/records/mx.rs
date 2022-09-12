use crate::{
    packing::UnpackError,
    types::rr::{RRHeader, Record, ResourceRecord},
};

/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.9
#[derive(Debug, Clone)]
pub struct MX {
    pub header: RRHeader,
    pub preference: u16,
    pub exchange: String,
}

impl MX {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {
            header,
            preference: 0,
            exchange: String::new(),
        }
        .into();
    }
}

impl Record for MX {
    fn header(&self) -> &RRHeader {
        return &self.header;
    }

    fn set_header(&mut self, header: RRHeader) {
        self.header = header;
    }

    fn len(&self) -> u16 {
        // TODO (Techassi): Labels length
        return (0 + 2) as u16;
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for MX {
    fn to_string(&self) -> String {
        format!("MX <PREF: {} - EX: {}>", self.preference, self.exchange)
    }
}

impl PartialEq<Self> for MX {
    fn eq(&self, other: &Self) -> bool {
        self.preference == other.preference && self.exchange == other.exchange
    }
}
