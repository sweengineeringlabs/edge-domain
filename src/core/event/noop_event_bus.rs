//! [`NoopEventBus`] — event bus that silently discards all events.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::broadcast;

use crate::api::event::domain_event::DomainEvent;
use crate::api::event::event_bus::EventBus;
use crate::api::error::EventError;
use crate::api::types::EventReceiver;

/// Event bus that discards all published events.
///
/// Use in tests that require an `EventBus` but have no interest in the events.
pub(crate) struct NoopEventBus;

impl EventBus for NoopEventBus {
    fn publish(&self, _event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }

    fn subscribe(&self) -> EventReceiver {
        let (_, rx) = broadcast::channel(1);
        EventReceiver(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    struct NoopEventBusEvent;
    impl DomainEvent for NoopEventBusEvent {
        fn event_type(&self) -> &str {
            "test.any"
        }
        fn aggregate_id(&self) -> &str {
            "id"
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    #[test]
    fn test_noop_event_bus_is_constructible() {
        let _ = NoopEventBus;
    }

    #[tokio::test]
    async fn test_publish_returns_ok() {
        assert!(NoopEventBus
            .publish(Arc::new(NoopEventBusEvent))
            .await
            .is_ok());
    }
}
