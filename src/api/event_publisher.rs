//! `EventPublisher` trait — contract for emitting domain events.

use async_trait::async_trait;

use crate::api::event::DomainEvent;
use crate::api::event_error::EventError;

/// Emits domain events to subscribers.
///
/// Implementations live in infrastructure crates (in-process bus,
/// message broker, etc.) — never in `edge-domain`.
///
/// ```rust,ignore
/// #[async_trait]
/// impl EventPublisher for InProcessEventBus {
///     async fn publish(&self, event: &dyn DomainEvent) -> Result<(), EventError> {
///         self.dispatch(event).await
///     }
/// }
/// ```
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Emit a domain event. Returns `Err` if delivery fails.
    async fn publish(&self, event: &dyn DomainEvent) -> Result<(), EventError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_publisher_is_object_safe() {
        fn _assert(_: &dyn EventPublisher) {}
    }
}
