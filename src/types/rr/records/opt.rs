use crate::types::rr::{RRHeader, Record, ResourceRecord};

/// See https://datatracker.ietf.org/doc/html/rfc6891#section-6.1
/// Many OPT header fields have a special usa case:
/// - Class: UDP payload size. RFC 6891 romoved the 512 byte size limit.
///   This field can set the maximum size in bytes (e.g. 4096)
/// - TTL: This 32 bit field is split up in:
///   - 1 octet extended RCODEs
///   - 1 octet EDNS version
///   - DO bit
///   - 15 reserved bits
#[derive(Debug, Clone)]
pub struct OPT {}

impl OPT {
    pub fn new_with_header(header: RRHeader) -> ResourceRecord {
        return Self {}.into();
    }
}

impl Record for OPT {
    fn header(&self) -> &RRHeader {
        todo!()
    }

    fn set_header(&mut self, header: RRHeader) {
        todo!()
    }

    fn len(&self) -> u16 {
        todo!()
    }

    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, crate::packing::UnpackError> {
        todo!()
    }

    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()> {
        todo!()
    }
}

impl ToString for OPT {
    fn to_string(&self) -> String {
        format!("OPT")
    }
}
