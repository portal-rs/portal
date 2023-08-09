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

use crate::{transfer::Request, Header, Message, MessageError};

pub trait Transport:
    Stream<Item = Result<Message, MessageError>> + Sink<Request, Error = Self::SinkError> + Unpin
{
    // Workaround for https://github.com/rust-lang/rust/issues/52662
    type SinkError: std::fmt::Debug + std::error::Error;
}

#[derive(Debug, Error)]
pub enum UdpDnsTransportError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

pub struct UdpDnsTransport {
    /// This buffer contains messages to be sent until they are actually sent
    buffer: Vec<Request>,
    writer: WriteBuffer,
    socket: UdpSocket,
}

impl Transport for UdpDnsTransport {
    type SinkError = UdpDnsTransportError;
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

impl Sink<Request> for UdpDnsTransport {
    type Error = UdpDnsTransportError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.buffer.is_empty() {
            return Poll::Ready(Ok(()));
        }

        Poll::Pending
    }

    fn start_send(mut self: Pin<&mut Self>, item: Request) -> Result<(), Self::Error> {
        self.buffer.push(item);

        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut bytes_sent = 0;

        while let Some(request) = self.buffer.pop() {
            request.message.write_be(&mut self.writer).unwrap();

            let bytes = self.writer.owned_bytes();
            let bytes_to_send = bytes.len();
            self.writer.clear();

            // while bytes_sent < bytes_to_send {
            //     match self
            //         .socket
            //         .poll_send_to(cx, bytes.as_slice(), request.target_socket_addr)
            //     {
            //         Poll::Ready(result) => {
            //             bytes_sent += result?;
            //         }
            //         // FIXME: This might be an endless loop if we never succeed to send the complete message
            //         Poll::Pending => continue,
            //     }
            // }

            // bytes_sent = 0;

            match self
                .socket
                .poll_send_to(cx, bytes.as_slice(), request.target_socket_addr)
            {
                Poll::Ready(_) => (),
                Poll::Pending => {
                    self.buffer.push(request);
                    return Poll::Pending;
                }
            }
        }

        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}

impl UdpDnsTransport {
    /// Creates a new UDP DNS transport. This internally uses an asynchronous
    /// [`UdpSocket`] to send and retrieve raw UDP datagrams which are
    /// interpreted as DNS messages. This transport implements [`Sink`], which
    /// first saves massages to be sent in an internal buffer which can contain
    /// at max `buffer_size` messages.
    pub fn new(socket: UdpSocket, buffer_size: usize) -> Self {
        let writer = WriteBuffer::new();

        Self {
            buffer: Vec::with_capacity(buffer_size),
            socket,
            writer,
        }
    }
}
