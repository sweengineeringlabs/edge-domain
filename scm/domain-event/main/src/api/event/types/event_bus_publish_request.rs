//! [`EventBusPublishRequest`] — request to broadcast an event to all subscribers.

use std::sync::Arc;

use crate::api::DomainEvent;

/// Request to publish `event` to every current [`EventBus`](crate::api::EventBus) subscriber.
pub struct EventBusPublishRequest {
    /// The event to broadcast.
    pub event: Arc<dyn DomainEvent>,
}
