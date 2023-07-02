use std::{
    pin::Pin,
    task::{Context, Poll},
};

use binbuf::{
    read::{ReadBuffer, Readable},
    write::{WriteBuffer, Writeable},
    BigEndian,
};
use futures::{Sink, Stream};
use thiserror::Error;
use tokio::{io::ReadBuf, net::UdpSocket};

use crate::{Header, Message, MessageError};

#[derive(Debug, Error)]
pub enum UdpDnsTransportError {}

pub struct UdpDnsTransport {
    send_buffer: WriteBuffer,
    socket: UdpSocket,
}

impl Stream for UdpDnsTransport {
    // TODO (Techassi): Switch out this error type to an error which wraps MessageError and Stream related errors
    type Item = Result<Message, MessageError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0u8; 512];
        let mut buf = ReadBuf::new(&mut buffer);

        match self.socket.poll_recv(cx, &mut buf) {
            Poll::Ready(res) => match res {
                Ok(_) => {
                    let mut buf = ReadBuffer::new(&buffer);

                    let header = match Header::read::<BigEndian>(&mut buf) {
                        Ok(header) => header,
                        Err(err) => return Poll::Ready(Some(Err(err.into()))),
                    };

                    match Message::read::<BigEndian>(&mut buf, header) {
                        Ok(msg) => Poll::Ready(Some(Ok(msg))),
                        Err(err) => return Poll::Ready(Some(Err(err))),
                    }
                }
                Err(_) => todo!(),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Item: Writeable> Sink<Item> for UdpDnsTransport {
    type Error = UdpDnsTransportError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // TODO (Techassi): This is naive
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        item.write_be(&mut self.send_buffer).unwrap();
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // TODO (Techassi): We need access to the target address in here

        todo!()
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}

impl UdpDnsTransport {
    pub fn new(socket: UdpSocket) -> Self {
        let send_buffer = WriteBuffer::new();

        Self {
            socket,
            send_buffer,
        }
    }
}
