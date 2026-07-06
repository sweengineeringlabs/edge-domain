//! [`EventBusSubscribeResponse`] — wrapper for a new event bus subscription.

use crate::api::EventReceiver;

/// Result of [`EventBus::subscribe`](crate::api::EventBus::subscribe).
pub struct EventBusSubscribeResponse {
    /// Owned handle over the new subscription.
    pub receiver: EventReceiver,
}
