use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    net::SocketAddr,
    pin::Pin,
    task::Poll,
    time::{Duration, Instant},
};

use futures::{channel::oneshot, Future, Sink, SinkExt, Stream, StreamExt};
use thiserror::Error;

use crate::{transfer::Request, Message, MessageError};

#[derive(Debug, Error)]
pub enum MultiplexError {
    #[error("multiplexer stream closed")]
    StreamClosed,
}

pub struct Multiplexer<T>
where
    T: Stream<Item = Result<Message, MessageError>> + Sink<Request> + Unpin,
    T::Error: std::fmt::Debug + std::error::Error,
{
    /// This defines the maximum number of messages received in one batch. When
    /// this number is reached, we break out of the receive loop and continue
    /// handling the received messages and also send outbound messages. This
    /// assures the server doesn't get flooeded wich requests and is thus never
    /// able to further handle the incoming requests and respond to previous
    /// requests.
    max_num_recv_messages: usize,

    /// This defines the write timeout for each answer to be sent. When the
    /// server fails to send the message during this duration, the message will
    /// be dropped. Clients who sent the associated request will then timeout
    /// on their own.
    write_timeout: Duration,

    /// This defines the read timeout for each sent message. When the server
    /// doesn't receive the corresponding answer during this time window, the
    /// request is marked as 'timed out' and will be removed from the inflight
    /// message queue.
    read_timeout: Duration,

    // === Management of active requests
    /// This keeps track of inflight requests.
    inflights: HashMap<u16, Option<oneshot::Sender<Result<Message, MessageError>>>>,

    /// This is the underlying transport to receive and send messages from and
    /// to. The multiplexer doesn't care what protocol is used here and thus
    /// many different network listeners / streams can be used, like UDPP, TCP
    /// or DoH / DoT.
    transport: T,
}

impl<T> Stream for Multiplexer<T>
where
    T: Stream<Item = Result<Message, MessageError>> + Sink<Request> + Unpin,
    T::Error: std::fmt::Debug + std::error::Error,
{
    type Item = Result<(), MultiplexError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        // Keep track of the number of received messages. This helps us to
        // immediatly wake up the multiplexer on the next event loop.
        let mut num_recv_messages = 0;

        // Receive at most the maximum number of messages defined in the
        // multiplexer. This prevents DoS attacks / floods.
        for _ in 0..self.max_num_recv_messages {
            match self.transport.poll_next_unpin(cx) {
                Poll::Ready(poll) => match poll {
                    Some(item) => match item {
                        Ok(message) => {
                            // We received a DNS message (answer from remote
                            // DNS server). Lookup the corresponding inflight
                            // request and use the mpsc channel to send back
                            // the received answer.

                            // First we update the message received counter
                            num_recv_messages += 1;

                            // Next look up the messages transaction ID in the
                            // inflights hash map. All received messages should
                            // have a corresponding request saved in the map.
                            // If this is not the case, we know the other end
                            // sent us a big pile of BS. The multiplexer will
                            // log this but will silently ignore the message
                            // from the senders POV.
                            match self.inflights.entry(message.transaction_id()) {
                                Occupied(mut request) => {
                                    // Send the answer to the inflight response
                                    // handler
                                    match request.get_mut().take() {
                                        Some(chan) => {
                                            // if let Err(err) = chan.send(Ok(message)) {
                                            //     println!(
                                            //         "Failed to send back received message: {}",
                                            //         err
                                            //     )
                                            // }
                                            chan.send(Ok(message)).unwrap()
                                        }
                                        None => {
                                            println!("The completing channel was already used")
                                        }
                                    }
                                }
                                Vacant(_) => {
                                    println!("Invalid request id: {}", message.transaction_id())
                                }
                            }
                        }
                        Err(_) => {
                            // We received some invalid blob of bytes which
                            // couldn't be parsed as a DNS message. In this
                            // case we just log the error and then proceed
                            // as if nothing ever happened :)
                            todo!()
                        }
                    },
                    None => {
                        // When we receive None, most likely the underlying
                        // transport was closed and isn't producing any
                        // more messages. In this case we should also terminate
                        // the multiplexer.
                        return Poll::Ready(None);
                    }
                },
                Poll::Pending => {
                    // Break out of the loop when there are no more messages
                    // to pull from the stream.
                    break;
                }
            }
        }

        // Like mentioned above, when we run 'out' of available message
        // slots in the current batch, we immediatly wake up the multiplexer
        // in the next event loop to keep processing incoming messages.
        if num_recv_messages == self.max_num_recv_messages {
            cx.waker().wake_by_ref()
        }

        Poll::Pending
    }
}

impl<T> Multiplexer<T>
where
    T: Stream<Item = Result<Message, MessageError>> + Sink<Request> + Unpin,
    T::Error: std::fmt::Debug + std::error::Error,
{
    pub fn new(transport: T) -> Self {
        MultiplexerBuilder::default().build(transport)
    }

    pub fn builder() -> MultiplexerBuilder {
        MultiplexerBuilder::default()
    }

    pub async fn send_message(
        &mut self,
        message: Message,
        target: SocketAddr,
    ) -> Result<MultiplexResponseStream, MultiplexError> {
        let (tx, rx) = oneshot::channel();
        let xid = message.transaction_id();

        let request = Request::new(message, target);

        match self.transport.send(request).await {
            Ok(_) => self.inflights.insert(xid, Some(tx)),
            Err(_) => todo!(),
        };

        Ok(MultiplexResponseStream::new(rx))
    }

    /// Drop all timed out requests
    fn drop_timeouts(&mut self) {
        todo!()
    }
}

pub struct MultiplexerBuilder {
    max_number_recv: usize,
    write_timeout: Duration,
    read_timeout: Duration,
}

impl Default for MultiplexerBuilder {
    fn default() -> Self {
        Self {
            write_timeout: Duration::from_secs(2),
            read_timeout: Duration::from_secs(2),
            max_number_recv: 128,
        }
    }
}

impl MultiplexerBuilder {
    pub fn with_max_number_recv(&mut self, max: usize) -> &mut Self {
        self.max_number_recv = max;
        self
    }

    pub fn with_write_timeout(&mut self, write_timeout: Duration) -> &mut Self {
        self.write_timeout = write_timeout;
        self
    }

    pub fn with_read_timeout(&mut self, read_timeout: Duration) -> &mut Self {
        self.read_timeout = read_timeout;
        self
    }

    pub fn build<T>(&self, transport: T) -> Multiplexer<T>
    where
        T: Stream<Item = Result<Message, MessageError>> + Sink<Request> + Unpin,
        T::Error: std::fmt::Debug + std::error::Error,
    {
        Multiplexer {
            max_num_recv_messages: self.max_number_recv,
            write_timeout: self.write_timeout,
            read_timeout: self.read_timeout,
            inflights: HashMap::new(),
            transport,
        }
    }
}

#[derive(Debug)]
pub struct MultiplexResponseStream {
    rx: oneshot::Receiver<Result<Message, MessageError>>,
    last: Instant,
}

impl Future for MultiplexResponseStream {
    type Output = Result<Message, MessageError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.rx.try_recv().unwrap() {
            Some(res) => Poll::Ready(res),
            None => {
                // NOTE (Techassi): Is there a better way to do this?
                if self.last.elapsed().as_millis() > 100 {
                    cx.waker().wake_by_ref();
                }
                Poll::Pending
            }
        }
    }
}

impl MultiplexResponseStream {
    pub fn new(rx: oneshot::Receiver<Result<Message, MessageError>>) -> Self {
        Self {
            last: Instant::now(),
            rx,
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::net::UdpSocket;

    use crate::{transfer::UdpDnsTransport, Class, Header, Name, Question, RType};

    use super::*;

    #[tokio::test]
    async fn simple_udp_multiplexer() {
        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        let transport = UdpDnsTransport::new(socket, 100);
        let mut mp = Multiplexer::new(transport);

        let header = Header::new(123);
        let mut message = Message::new_with_header(header);

        let question = Question::new(Name::try_from("example.com").unwrap(), RType::A, Class::IN);
        message.add_question(question);

        let resp = mp
            .send_message(message, "1.1.1.1:53".parse().unwrap())
            .await
            .unwrap();

        tokio::select! {
            _ = mp.next() => {
                panic!("AHHHHH")
            },
            r = resp => {
                match r {
                    Ok(msg) => println!("{}", msg),
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
    }
}
