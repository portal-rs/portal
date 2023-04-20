use std::fmt::Display;

use binbuf::prelude::*;
use thiserror::Error;

use crate::{
    constants,
    resolver::ResultRecords,
    types::{
        dns::{Header, HeaderError, Question, QuestionError},
        rcode::Rcode,
        rr::{RData, Record, RecordError, SOA},
    },
};

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Header error: {0}")]
    HeaderError(#[from] HeaderError),

    #[error("Question error: {0}")]
    QuestionError(#[from] QuestionError),

    #[error("Record error: {0}")]
    RecordError(#[from] RecordError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

/// [`Message`] describes a complete DNS message describes in RFC 1035
/// Section 4. See https://datatracker.ietf.org/doc/html/rfc1035#section-4
#[derive(Debug, Default)]
pub struct Message {
    header: Header,
    question: Vec<Question>,
    answers: Vec<Record>,
    authorities: Vec<Record>,
    additionals: Vec<Record>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let answers: String = self.answers.iter().map(|r| format!("{r}\n")).collect();
        let authority: String = self.authorities.iter().map(|r| format!("{r}\n")).collect();
        let additional: String = self.additionals.iter().map(|r| format!("{r}\n")).collect();

        write!(
            f,
            ";; ->>HEADER<<- opcode: {}, rcode: {}, id: {}\n\
            ;; QUESTION SECTION:\n\
            ;; {}\n\n\
            ;; ANSWER SECTION:\n{}\n\
            ;; AUTHORITY SECTION:\n{}\n\
            ;; ADDITIONAL SECTION:\n{}",
            self.header.opcode,
            self.header.rcode,
            self.header.id,
            self.question().unwrap(),
            answers,
            authority,
            additional
        )
    }
}

impl Writeable for Message {
    type Error = MessageError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.header.write::<E>(buf)?;

            self.question.write::<E>(buf)?;
            self.answers.write::<E>(buf)?;
            self.authorities.write::<E>(buf)?;
            self.additionals.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl Message {
    /// Return a new default [`Message`] with the provided [`Header`] already set.
    pub fn new_with_header(header: Header) -> Self {
        Self {
            header,
            ..Default::default()
        }
    }

    /// Set the vector of questions to the provided one. This does **NOT**
    /// update the QDCOUNT in the DNS header. This method is only usable
    /// within the library it self, as this is potentially dangerous and/or
    /// can cause faulty behavior.
    ///
    /// To instead correctly update the QDCOUNT use the methods:
    ///
    /// - [`Message::add_question`](Message#method.add_question)
    /// - [`Message::add_questions`](Message#method.add_questions)
    pub(crate) fn set_questions(&mut self, questions: Vec<Question>) {
        self.question = questions;
    }

    /// Adds one question to the question section and updates the QDCOUNT in
    /// the DNS header.
    pub fn add_question(&mut self, question: Question) {
        self.question.push(question);
        self.header.qdcount += 1;
    }

    pub fn question(&self) -> Option<&Question> {
        if !self.question.is_empty() {
            return Some(&self.question[0]);
        }
        None
    }

    /// Returns a reference to the list of questions.
    pub fn questions(&self) -> &Vec<Question> {
        &self.question
    }

    /// Set the vector of answer records to the provided one. This does
    /// **NOT** update the ANCOUNT in the DNS header. This method is only
    /// usable within the library it self, as this is potentially dangerous
    /// and/or can cause faulty behavior.
    ///
    /// To instead correctly update the ANCOUNT use the methods:
    ///
    /// - [`Message::add_question`](Message#method.add_question)
    /// - [`Message::add_questions`](Message#method.add_questions)
    pub(crate) fn set_answers(&mut self, answers: Vec<Record>) {
        self.answers = answers;
    }

    /// Adds one answer to the answer section and updates the ANCOUNT in the
    /// DNS header.
    pub fn add_answer(&mut self, answer: Record) {
        self.answers.push(answer);
        self.header.ancount += 1;
    }

    /// Adds multiple answers to the answer section and updates the ANCOUNT in
    /// the DNS header.
    pub fn add_answers(&mut self, answers: &mut Vec<Record>) {
        self.header.ancount += answers.len() as u16;
        self.answers.append(answers);
    }

    /// Returns a reference to the list of RRs in the answer section.
    pub fn answers(&self) -> &Vec<Record> {
        &self.answers
    }

    /// Set the vector of authority records to the provided one. This does
    /// **NOT** update the NSCOUNT in the DNS header. This method is only
    /// usable within the library it self, as this is potentially dangerous
    /// and/or can cause faulty behavior.
    ///
    /// To instead correctly update the NSCOUNT use the methods:
    ///
    /// - [`Message::add_authority`](Message#method.add_authority)
    /// - [`Message::add_authorities`](Message#method.add_authorities)
    pub(crate) fn set_authorities(&mut self, authorities: Vec<Record>) {
        self.authorities = authorities;
    }

    /// Adds one authority record to the authority section and updates the
    /// NSCOUNT in the DNS header.
    pub fn add_authority(&mut self, authority: Record) {
        self.authorities.push(authority);
        self.header.nscount += 1;
    }

    /// Adds multiple authority record to the authority section and updates
    /// the NSCOUNT in the DNS header.
    pub fn add_authorities(&mut self, authorities: &mut Vec<Record>) {
        self.header.nscount += authorities.len() as u16;
        self.authorities.append(authorities);
    }

    /// Returns a reference to the list of RRs in the authority section.
    pub fn authorities(&self) -> &Vec<Record> {
        &self.authorities
    }

    /// Set the vector of additional records to the provided one. This does
    /// **NOT** update the ARCOUNT in the DNS header. This method is only
    /// usable within the library it self, as this is potentially dangerous
    /// and/or can cause faulty behavior.
    ///
    /// To instead correctly update the ARCOUNT use the methods:
    ///
    /// - [`Message::add_additional`](Message#method.add_additional)
    /// - [`Message::add_additionals`](Message#method.add_additionals)
    pub fn set_additionals(&mut self, additionals: Vec<Record>) {
        self.additionals = additionals;
    }

    /// Adds one additional record to the additional section and updates the
    /// ARCOUNT in the DNS header.
    pub fn add_additional(&mut self, additional: Record) {
        self.additionals.push(additional);
        self.header.arcount += 1;
    }

    /// Adds multiple additional record to the additional section and updates
    /// the ARCOUNT in the DNS header.
    pub fn add_additionals(&mut self, additionals: &mut Vec<Record>) {
        self.header.arcount += additionals.len() as u16;
        self.additionals.append(additionals);
    }

    /// Returns a reference to the list of RRs in the additional section.
    pub fn additionals(&self) -> &Vec<Record> {
        &self.additionals
    }

    /// Add all result records from a query to the correct RR sections. This
    /// updates all RR counts in the DNS header.
    pub fn add_query_result(&mut self, result: &mut ResultRecords) {
        self.add_answers(&mut result.answers);
        self.add_authorities(&mut result.authorities);
        self.add_additionals(&mut result.additionals);
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
        self.header.qdcount
    }

    /// Returns ANCOUNT stored in the DNS message header.
    pub fn ancount(&self) -> u16 {
        self.header.ancount
    }

    /// Returns NSCOUNT stored in the DNS message header.
    pub fn nscount(&self) -> u16 {
        self.header.nscount
    }

    /// Returns ARCOUNT stored in the DNS message header.
    pub fn arcount(&self) -> u16 {
        self.header.arcount
    }

    /// Returns the size of this DNS [`Message`].
    pub fn size(&self) -> usize {
        let mut len = constants::dns::HEADER_LENGTH;
        len += self.question[0].size();

        for answer in &self.answers {
            len += answer.size();
        }

        for authority in &self.authorities {
            len += authority.size();
        }

        for additional in &self.additionals {
            len += additional.size();
        }

        len
    }

    /// Returns if the message contains any SOA RRs in the authoritative
    /// section.
    pub fn is_soa(&self) -> bool {
        for record in &self.authorities {
            if record.is_soa() {
                return true;
            }
        }

        false
    }

    pub fn get_soa_record(&self) -> Option<&SOA> {
        for record in &self.authorities {
            match record.rdata() {
                RData::SOA(soa) => return Some(soa),
                _ => continue,
            };

            // cast_or!(record.get_rdata(), RData::SOA, continue);
        }

        None
    }

    /// Returns if the message contains any EDNS options stored in OPT records.
    /// This functions looks at records in the additional section from the
    /// back, because the OPT RRs are usually at the end of this section.
    pub fn is_edns(&self) -> bool {
        for record in self.additionals.iter().rev() {
            if record.is_edns() {
                return true;
            }
        }

        false
    }

    /// Sets the RCODE of the message
    pub fn set_rcode(&mut self, rcode: Rcode) {
        self.header.rcode = rcode
    }

    /// Read the complete DNS [`Message`] based on the already unpacked [`Header`].
    pub fn read<E: Endianness>(buf: &mut ReadBuffer, header: Header) -> Result<Self, MessageError> {
        let mut message = Self::new_with_header(header);

        // Read questions
        let questions = read_questions::<E>(buf, message.qdcount())?;
        message.set_questions(questions);

        // Read answer records. This will most likely be empty for requests
        let answers = read_rrs::<E>(buf, message.ancount())?;
        message.set_answers(answers);

        // Read authority records
        let authorities = read_rrs::<E>(buf, message.nscount())?;
        message.set_authorities(authorities);

        // Read additional records
        let additionals = read_rrs::<E>(buf, message.arcount())?;
        message.set_additionals(additionals);

        Ok(message)
    }
}

fn read_questions<E: Endianness>(
    buf: &mut ReadBuffer,
    count: u16,
) -> Result<Vec<Question>, QuestionError> {
    let mut questions: Vec<Question> = Vec::new();

    // Let's do a naive approach and assume the QDCOUNT is correct
    // TODO (Techassi): Don't be naive
    for _ in 0..count {
        match Question::read::<E>(buf) {
            Ok(question) => questions.push(question),
            Err(err) => return Err(err),
        };
    }

    Ok(questions)
}

fn read_rrs<E: Endianness>(buf: &mut ReadBuffer, count: u16) -> Result<Vec<Record>, RecordError> {
    let mut rrs: Vec<Record> = Vec::new();

    for _ in 0..count {
        match Record::read::<E>(buf) {
            Ok(rr) => rrs.push(rr),
            Err(err) => return Err(err),
        }
    }

    Ok(rrs)
}
