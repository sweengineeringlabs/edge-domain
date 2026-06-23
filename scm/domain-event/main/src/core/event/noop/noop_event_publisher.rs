//! [`EventPublisher`] impl for [`NoopEventPublisher`] — silently discards all events.

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::{DomainEvent, EventPublisher};
use crate::api::NoopEventPublisher;

impl EventPublisher for NoopEventPublisher {
    fn publish(&self, _event: &dyn DomainEvent) -> BoxFuture<'_, Result<(), EventError>> {
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
        assert_eq!(result, Ok(()));
    }

    /// @covers: publish
    #[test]
    fn test_publish_via_dyn_returns_ok_error() {
        let pub_: &dyn EventPublisher = &NoopEventPublisher;
        let evt: &dyn DomainEvent = &NoopEventPublisherTestEvt;
        let result = futures::executor::block_on(pub_.publish(evt));
        assert_eq!(result, Ok(()));
    }

    /// @covers: publish
    #[test]
    fn test_publish_repeated_calls_all_ok_edge() {
        for _ in 0..5 {
            assert_eq!(futures::executor::block_on(NoopEventPublisher.publish(&NoopEventPublisherTestEvt)), Ok(()));
        }
    }
}
