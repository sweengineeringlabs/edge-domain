//! [`EventBusSubscribeResponse`] — wrapper for a new event bus subscription.

use crate::api::EventSource;

/// Result of [`EventBus::subscribe`](crate::api::EventBus::subscribe).
pub struct EventBusSubscribeResponse {
    /// Owned, type-erased handle over the new subscription. Call
    /// [`recv`](EventSource::recv) to pull the next event.
    pub receiver: Box<dyn EventSource>,
}
