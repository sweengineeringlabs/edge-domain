//! `impl EventReceiver` — construction and `recv` convenience method.

use std::sync::Arc;

use crate::api::EventError;
use crate::api::{DomainEvent, EventReceiver, EventSource, EventSourceRecvNextRequest};

impl EventReceiver {
    /// Wrap any [`EventSource`] in a receiver.
    pub fn new(source: impl EventSource + 'static) -> Self {
        Self(Box::new(source))
    }

    /// Pull the next event from the source, waiting asynchronously.
    pub async fn recv(&mut self) -> Result<Arc<dyn DomainEvent>, EventError> {
        self.0
            .recv_next(EventSourceRecvNextRequest)
            .await
            .map(|resp| resp.event)
    }
}
