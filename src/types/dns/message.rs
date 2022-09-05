use crate::types::rr::RR;

use super::{Header, Question};

/// [`Message`] describes a complete DNS message describes in RFC 1035
/// Section 4. See https://datatracker.ietf.org/doc/html/rfc1035#section-4
#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<RR>,
    pub authority: Vec<RR>,
    pub additional: Vec<RR>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header::default(),
            question: Vec::new(),
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        }
    }
}

impl Message {
    /// Return a new default [`Message`] with the provded [`Header`] already set.
    pub fn new_with_header(header: Header) -> Self {
        return Self {
            header,
            ..Default::default()
        };
    }
}
