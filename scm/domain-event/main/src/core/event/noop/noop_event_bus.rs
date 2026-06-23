//! [`EventBus`] impl for [`NoopEventBus`] — discards all events.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::{DomainEvent, EventBus, EventSource};
use crate::api::{EventReceiver, NoopEventBus};

/// Private source returned by [`NoopEventBus::subscribe`] — immediately closed.
struct NoopEventBusSource;

impl EventSource for NoopEventBusSource {
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>> {
        Box::pin(async { Err(EventError::Unavailable("noop bus has no events".into())) })
    }
}

impl EventBus for NoopEventBus {
    fn publish(&self, _event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }

    fn subscribe(&self) -> EventReceiver {
        EventReceiver::new(NoopEventBusSource)
    }
}

