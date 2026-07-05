//! [`EventPublisherPublishRequest`] — request to publish a single event.

use crate::api::DomainEvent;

/// Request to publish `event` via an [`EventPublisher`](crate::api::EventPublisher).
pub struct EventPublisherPublishRequest<'a> {
    /// The event to publish.
    pub event: &'a dyn DomainEvent,
}
