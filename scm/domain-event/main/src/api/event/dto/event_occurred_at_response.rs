//! [`EventOccurredAtResponse`] — wrapper for an event's occurrence time.

use std::time::SystemTime;

/// Result of [`DomainEvent::occurred_at`](crate::api::DomainEvent::occurred_at).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventOccurredAtResponse {
    /// Wall-clock time at which the event occurred.
    pub occurred_at: SystemTime,
}
