use async_trait::async_trait;
use binbuf::prelude::*;
use portal_proto::{Header, HeaderError, Message, MessageError, Opcode, QuestionError};
use thiserror::Error;

use crate::accept::Action;

#[derive(Debug, Error)]
pub enum RequestHandlerError {
    #[error("Header error: {0}")]
    HeaderError(#[from] HeaderError),

    #[error("Question error: {0}")]
    QuestionError(#[from] QuestionError),

    #[error("Message error: {0}")]
    MessageError(#[from] MessageError),
}

#[async_trait]
pub trait RequestHandler: Send + Sync + 'static {
    /// Receive a DNS request. The default implementation reads the data with a
    /// ReadBuffer, unpacks the DNS header and passes the header to the
    /// `should_accept` method. This method validates some basic header fields.
    async fn receive_request(&self, buf: &[u8]) -> Result<Option<Message>, RequestHandlerError> {
        let mut buf = ReadBuffer::new(buf);

        let header = Header::read::<BigEndian>(&mut buf)?;

        match self.should_accept(header) {
            Action::Accept => self.handle_accept(&mut buf, header).await,
            Action::Reject => self.handle_reject(&mut buf, header).await,
            Action::Ignore => self.handle_ignore(&mut buf, header).await,
            Action::NoImpl => self.handle_noimpl(&mut buf, header).await,
        }
    }

    /// This methods validates basic header fields and then decides which
    /// action to take.
    fn should_accept(&self, header: Header) -> Action {
        if !header.is_query {
            return Action::Ignore;
        }

        // If there is no question section at all
        if header.qdcount == 0 {
            return Action::Ignore;
        }

        if header.opcode != Opcode::Query {
            return Action::NoImpl;
        }

        // If there is more than one question, we reject. Most DNS Servers and
        // resolvers don't implement this feature.
        if header.qdcount > 1 {
            return Action::Reject;
        }

        Action::Accept
    }

    /// This method handles accepted messages
    async fn handle_accept(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError> {
        let msg = Message::read::<BigEndian>(buf, header)?;
        Ok(Some(msg))
    }

    /// This method handles rejected messages
    async fn handle_reject(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError>;

    /// This method handles ignored messages
    async fn handle_ignore(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError>;

    /// This method handles noimpl messages
    async fn handle_noimpl(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError>;
}
