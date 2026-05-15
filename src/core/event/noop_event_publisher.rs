//! `NoopEventPublisher` — discards events silently.

use async_trait::async_trait;

use crate::api::event::DomainEvent;
use crate::api::event::EventError;
use crate::api::event::EventPublisher;

/// Accepts events and discards them without side effects.
///
/// Use during development, testing, or in services that do not yet
/// require event publishing infrastructure.
pub(crate) struct NoopEventPublisher;

#[async_trait]
impl EventPublisher for NoopEventPublisher {
    async fn publish(&self, _event: &dyn DomainEvent) -> Result<(), EventError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    struct NoopEventPublisherEvent;
    impl DomainEvent for NoopEventPublisherEvent {
        fn event_type(&self)   -> &str       { "any" }
        fn aggregate_id(&self) -> &str       { "id-1" }
        fn occurred_at(&self)  -> SystemTime { SystemTime::now() }
    }

    /// @covers: publish
    #[tokio::test]
    async fn test_publish_always_returns_ok() {
        assert!(NoopEventPublisher.publish(&NoopEventPublisherEvent).await.is_ok());
    }
}
