use crate::{
    constants,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::rr::Record,
};

use super::{Header, Question};

/// [`Message`] describes a complete DNS message describes in RFC 1035
/// Section 4. See https://datatracker.ietf.org/doc/html/rfc1035#section-4
#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header::default(),
            question: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }
}

impl Packable for Message {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.header.pack(buf)?;

        pack_questions(buf, &self.question)?;
        pack_rrs(buf, &self.answers)?;
        pack_rrs(buf, &self.authorities)?;
        pack_rrs(buf, &self.additionals)?;

        Ok(())
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

    pub fn add_question(&mut self, question: Question) {
        self.question.push(question);
        self.header.qdcount += 1;
    }

    pub fn set_answers(&mut self, answers: Vec<Record>) {
        self.answers = answers;
    }

    pub fn set_authorities(&mut self, authorities: Vec<Record>) {
        self.authorities = authorities;
    }

    pub fn set_additionals(&mut self, additionals: Vec<Record>) {
        self.additionals = additionals;
    }

    /// Set if the message is a response.
    pub fn set_is_response(&mut self, is_response: bool) {
        self.header.is_query = !is_response;
    }

    /// Set if recursion is available.
    pub fn set_rec_avail(&mut self, avail: bool) {
        self.header.rec_avail = avail;
    }

    /// Returns QDCOUNT stored in the DNS message header.
    pub fn qdcount(&self) -> u16 {
        return self.header.qdcount;
    }

    /// Returns ANCOUNT stored in the DNS message header.
    pub fn ancount(&self) -> u16 {
        return self.header.ancount;
    }

    /// Returns NSCOUNT stored in the DNS message header.
    pub fn nscount(&self) -> u16 {
        return self.header.nscount;
    }

    /// Returns ARCOUNT stored in the DNS message header.
    pub fn arcount(&self) -> u16 {
        return self.header.arcount;
    }

    /// Returns the length of this DNS [`Message`].
    pub fn len(&self) -> usize {
        let mut len = constants::dns::HEADER_LENGTH;
        len += self.question[0].len();

        for answer in &self.answers {
            len += answer.len();
        }

        for authority in &self.authorities {
            len += authority.len();
        }

        for additional in &self.additionals {
            len += additional.len();
        }

        return len;
    }

    /// Unpack the complete DNS [`Message`] based on the already unpacked [`Header`].
    pub fn unpack(buf: &mut UnpackBuffer, header: Header) -> UnpackBufferResult<Self> {
        let mut message = Self::new_with_header(header);

        // Unpack questions
        let questions = unpack_questions(buf, message.qdcount())?;
        message.set_questions(questions);

        // Unpack answer records. This will most likey be empty for requests
        let answers = unpack_rrs(buf, message.ancount())?;
        message.set_answers(answers);

        // Unpack authority records
        let authorities = unpack_rrs(buf, message.nscount())?;
        message.set_authorities(authorities);

        // Unpack additional records
        let additionals = unpack_rrs(buf, message.arcount())?;
        message.set_additionals(additionals);

        Ok(message)
    }
}

fn unpack_questions(buf: &mut UnpackBuffer, count: u16) -> UnpackBufferResult<Vec<Question>> {
    let mut questions: Vec<Question> = Vec::new();

    // Let's do a naive approach and assume the QDCOUNT is correct
    // TODO (Techassi): Don't be naive
    for _ in 0..count {
        match Question::unpack(buf) {
            Ok(question) => questions.push(question),
            Err(err) => return Err(err),
        };
    }

    Ok(questions)
}

fn unpack_rrs(buf: &mut UnpackBuffer, count: u16) -> UnpackBufferResult<Vec<Record>> {
    let mut rrs: Vec<Record> = Vec::new();

    for _ in 0..count {
        match Record::unpack(buf) {
            Ok(rr) => rrs.push(rr),
            Err(err) => return Err(err),
        }
    }

    Ok(rrs)
}

fn pack_questions(buf: &mut PackBuffer, questions: &Vec<Question>) -> PackBufferResult {
    for question in questions {
        question.pack(buf)?;
    }

    Ok(())
}

fn pack_rrs(buf: &mut PackBuffer, records: &Vec<Record>) -> PackBufferResult {
    for record in records {
        record.pack(buf)?;
    }

    Ok(())
}
