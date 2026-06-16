//! [`EventReceiver`] — owned handle over an [`EventSource`].

use std::sync::Arc;

use crate::api::event::errors::EventError;
use crate::api::event::traits::{DomainEvent, EventSource};

/// An owned, type-erased handle over an [`EventSource`].
///
/// Obtained from [`EventBus::subscribe`](super::super::traits::EventBus::subscribe).
/// Call [`recv`](EventReceiver::recv) to pull the next event from the underlying source.
pub struct EventReceiver(Box<dyn EventSource>);

impl EventReceiver {
    /// Wrap any [`EventSource`] in a receiver.
    pub fn new(source: impl EventSource + 'static) -> Self {
        Self(Box::new(source))
    }

    /// Pull the next event from the source, waiting asynchronously.
    pub async fn recv(&mut self) -> Result<Arc<dyn DomainEvent>, EventError> {
        self.0.recv_next().await
    }
}
