//! [`NoopEventPublisher`] — discards events silently.

use futures::future::BoxFuture;

use crate::api::event::DomainEvent;
use crate::api::event::EventError;
use crate::api::event::EventPublisher;

/// Accepts events and discards them without side effects.
pub(crate) struct NoopEventPublisher;

impl EventPublisher for NoopEventPublisher {
    fn publish<'a>(&'a self, _event: &'a dyn DomainEvent) -> BoxFuture<'a, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    struct NoopEventPublisherEvent;
    impl DomainEvent for NoopEventPublisherEvent {
        fn event_type(&self) -> &str {
            "any"
        }
        fn aggregate_id(&self) -> &str {
            "id-1"
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    #[test]
    fn test_noop_event_publisher_is_constructible() {
        let _ = NoopEventPublisher;
    }

    #[tokio::test]
    async fn test_publish_returns_ok() {
        assert!(NoopEventPublisher
            .publish(&NoopEventPublisherEvent)
            .await
            .is_ok());
    }
}
