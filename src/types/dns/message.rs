use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, UnpackError, Unpackable},
    types::rr::Record,
};

use super::{Header, Question};

/// [`Message`] describes a complete DNS message describes in RFC 1035
/// Section 4. See https://datatracker.ietf.org/doc/html/rfc1035#section-4
#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<Record>,
    pub authority: Vec<Record>,
    pub additional: Vec<Record>,
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

    pub fn set_questions(&mut self, questions: Vec<Question>) {
        self.question = questions;
    }

    /// Unpack the complete DNS [`Message`] based on the already unpacked [`Header`].
    pub fn unpack(buf: &mut UnpackBuffer, header: Header) -> UnpackBufferResult<Self> {
        let mut message = Self::new_with_header(header);

        match message.unpack_questions(buf) {
            Ok(questions) => message.set_questions(questions),
            Err(err) => return Err(err),
        }

        Ok(message)
    }

    fn unpack_questions(&self, buf: &mut UnpackBuffer) -> UnpackBufferResult<Vec<Question>> {
        let mut questions: Vec<Question> = Vec::new();
        let count = self.header.qdcount;

        // Let's do a naive approach and assume the QDCOUNT is correct
        // TODO (Techassi): Don't be naive
        for i in 0..count {
            match Question::unpack(buf) {
                Ok(question) => questions.push(question),
                Err(err) => return Err(err),
            };
        }

        Ok(questions)
    }

    fn unpack_rrs(&self, buf: &mut UnpackBuffer) -> UnpackBufferResult<Vec<Record>> {
        let mut rrs: Vec<Record> = Vec::new();
        Ok(rrs)
    }
}
