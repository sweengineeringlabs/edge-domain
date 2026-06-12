//! [`EventBus`] impl for [`NoopEventBus`] — discards all events.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::{DomainEvent, EventBus, EventSource};
use crate::api::event::types::{EventReceiver, NoopEventBus};

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

#[cfg(test)]
mod tests {
    use super::*;

    struct NoopEventBusTestEvt;
    impl DomainEvent for NoopEventBusTestEvt {}

    /// @covers: publish
    #[test]
    fn test_publish_any_event_returns_ok_happy() {
        let result =
            futures::executor::block_on(NoopEventBus.publish(Arc::new(NoopEventBusTestEvt)));
        assert!(result.is_ok());
    }

    /// @covers: subscribe
    #[test]
    fn test_subscribe_receiver_immediately_unavailable_error() {
        let mut rx = NoopEventBus.subscribe();
        let result = futures::executor::block_on(rx.recv());
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    }

    /// @covers: publish
    #[test]
    fn test_publish_repeated_calls_all_ok_edge() {
        for _ in 0..5 {
            assert!(
                futures::executor::block_on(NoopEventBus.publish(Arc::new(NoopEventBusTestEvt))).is_ok()
            );
        }
    }
}
