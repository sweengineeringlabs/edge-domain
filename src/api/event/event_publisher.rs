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
}
