use async_trait::async_trait;
use binbuf::prelude::*;
use portal_proto::{Header, Message, Rcode};
use portal_resolver::ToResolver;

use crate::{
    cache::Cache,
    request::{RequestHandler, RequestHandlerError},
};

pub struct DefaultRequestHandler {}

#[async_trait]
impl RequestHandler for DefaultRequestHandler {
    async fn handle_reject(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError> {
        let mut msg = Message::read::<BigEndian>(buf, header)?;
        msg.set_rcode(Rcode::Refused);

        Ok(Some(msg))
    }

    async fn handle_ignore(
        &self,
        _buf: &mut ReadBuffer,
        _header: Header,
    ) -> Result<Option<Message>, RequestHandlerError> {
        Ok(None)
    }

    async fn handle_noimpl(
        &self,
        buf: &mut ReadBuffer,
        header: Header,
    ) -> Result<Option<Message>, RequestHandlerError> {
        let mut msg = Message::read::<BigEndian>(buf, header)?;
        msg.set_rcode(Rcode::NotImpl);

        Ok(Some(msg))
    }
}

impl DefaultRequestHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct DefaultResponseHandler<T: ToResolver + Sync + Send + 'static> {
    cache: Cache,
    resolver: T,
}

impl<T: ToResolver + Sync + Send + 'static> DefaultResponseHandler<T> {
    pub fn new(cache: Cache, resolver: T) -> Self {
        Self { cache, resolver }
    }
}
