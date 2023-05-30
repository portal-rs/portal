use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    pin::Pin,
    task::Poll,
    time::Duration,
};

use futures::{channel::oneshot::Sender, Sink, Stream, StreamExt};

use crate::{Message, MessageError};

pub trait MultiplexMessage {
    fn transaction_id(&self) -> u16;
}

pub enum MultiplexerError {
    StreamClosed,
}

pub struct Multiplexer<T, M>
where
    T: Stream + Sink<T::Item> + Unpin,
    M: MultiplexMessage,
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
    inflights: HashMap<u16, Sender<M>>,

    /// This is the underlying transport to receive and send messages from and
    /// to. The multiplexer doesn't care what protocol is used here and thus
    /// many different network listeners / streams can be used, like UDPP, TCP
    /// or DoH / DoT.
    transport: T,
}

impl<T, M> Stream for Multiplexer<T, M>
where
    T: Stream<Item = Result<M, MessageError>> + Sink<T::Item> + Unpin,
    M: MultiplexMessage,
{
    type Item = Result<Message, MessageError>;

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
                            // DNS server). Lookup the corresponf inflight
                            // request and use the oneshot channel to send back
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
                                Occupied(mut _request) => {
                                    // Send the answer to the inflight response
                                    // handler
                                    // let r = request.get_mut().send(message);
                                }
                                Vacant(_) => todo!(),
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
                        todo!()
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

        todo!()
    }
}

impl<T, M> Multiplexer<T, M>
where
    T: Stream<Item = Result<M, MessageError>> + Sink<T::Item> + Unpin,
    M: MultiplexMessage,
{
    pub fn new(transport: T) -> Self {
        MultiplexerBuilder::default().build(transport)
    }

    pub fn builder() -> MultiplexerBuilder {
        MultiplexerBuilder::default()
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

    pub fn build<T, M>(&self, transport: T) -> Multiplexer<T, M>
    where
        T: Stream<Item = Result<M, MessageError>> + Sink<T::Item> + Unpin,
        M: MultiplexMessage,
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
