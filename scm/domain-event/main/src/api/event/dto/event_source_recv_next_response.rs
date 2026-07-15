//! [`EventSourceRecvNextResponse`] — wrapper for the next available event.

use std::sync::Arc;

use crate::api::DomainEvent;

/// Result of [`EventSource::recv_next`](crate::api::EventSource::recv_next).
pub struct EventSourceRecvNextResponse {
    /// The next available event.
    pub event: Arc<dyn DomainEvent>,
}
