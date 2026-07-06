//! `EventPublisher` trait — fire-and-forget event publishing contract.

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::types::EventPublisherPublishRequest;

/// Publishes a single [`DomainEvent`](super::DomainEvent) without returning a subscription handle.
///
/// This is the simpler outbound port for event emission when the caller does
/// not need to receive events back.
pub trait EventPublisher: Send + Sync {
    /// Publish one event.
    fn publish(&self, req: EventPublisherPublishRequest<'_>) -> BoxFuture<'_, Result<(), EventError>>;
}
