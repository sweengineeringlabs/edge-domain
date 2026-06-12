//! [`EventPublisher`] impl for [`NoopEventPublisher`] — silently discards all events.

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::{DomainEvent, EventPublisher};
use crate::api::event::types::NoopEventPublisher;

impl EventPublisher for NoopEventPublisher {
    fn publish<'a>(&'a self, _event: &'a dyn DomainEvent) -> BoxFuture<'a, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoopEventPublisherTestEvt;
    impl DomainEvent for NoopEventPublisherTestEvt {}

    /// @covers: publish
    #[test]
    fn test_publish_any_event_returns_ok_happy() {
        let result = futures::executor::block_on(NoopEventPublisher.publish(&NoopEventPublisherTestEvt));
        assert!(result.is_ok());
    }

    /// @covers: publish
    #[test]
    fn test_publish_via_dyn_returns_ok_error() {
        let pub_: &dyn EventPublisher = &NoopEventPublisher;
        let evt: &dyn DomainEvent = &NoopEventPublisherTestEvt;
        let result = futures::executor::block_on(pub_.publish(evt));
        assert!(result.is_ok());
    }

    /// @covers: publish
    #[test]
    fn test_publish_repeated_calls_all_ok_edge() {
        for _ in 0..5 {
            assert!(futures::executor::block_on(NoopEventPublisher.publish(&NoopEventPublisherTestEvt)).is_ok());
        }
    }
}
