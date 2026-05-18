//! `EventPublisher` trait — contract for emitting domain events.

use futures::future::BoxFuture;

use super::domain_event::DomainEvent;
use super::event_error::EventError;

/// Emits domain events to subscribers.
///
/// Implementations live in infrastructure crates (in-process bus,
/// message broker, etc.) — never in `edge-domain`.
///
/// ```rust,ignore
/// impl EventPublisher for InProcessEventBus {
///     fn publish<'a>(&'a self, event: &'a dyn DomainEvent) -> BoxFuture<'a, Result<(), EventError>> {
///         Box::pin(async move { self.dispatch(event).await })
///     }
/// }
/// ```
pub trait EventPublisher: Send + Sync {
    /// Emit a domain event. Returns `Err` if delivery fails.
    fn publish<'a>(&'a self, event: &'a dyn DomainEvent) -> BoxFuture<'a, Result<(), EventError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_publisher_is_object_safe() {
        fn _assert(_: &dyn EventPublisher) {}
    }

    struct NoopPublisher;
    impl EventPublisher for NoopPublisher {
        fn publish<'a>(
            &'a self,
            _event: &'a dyn DomainEvent,
        ) -> BoxFuture<'a, Result<(), EventError>> {
            Box::pin(async { Ok(()) })
        }
    }

    struct AnyEvent;
    impl DomainEvent for AnyEvent {
        fn event_type(&self) -> &str {
            "any"
        }
        fn aggregate_id(&self) -> &str {
            "id"
        }
        fn occurred_at(&self) -> std::time::SystemTime {
            std::time::SystemTime::now()
        }
    }

    #[tokio::test]
    async fn test_publish_returns_ok() {
        assert!(NoopPublisher.publish(&AnyEvent).await.is_ok());
    }
}
