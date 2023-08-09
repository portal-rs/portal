use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

use crate::{
    transfer::{Multiplexer, Transport},
    Message,
};

/// A [`Background<T>`] runs a stream in the background, for example the
/// multiplexer. [`Background`] implements [`Future`] which allows external
/// callers to spawn the future within a Tokio task. This is what drives the
/// [`Background`] (and the internal stream / multiplexer) forward.
pub struct Background<T>
where
    T: Transport,
{
    pub enqueued_messages: VecDeque<Message>,
    pub stream: Multiplexer<T>,
}

impl<S> Future for Background<S>
where
    S: Transport,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
