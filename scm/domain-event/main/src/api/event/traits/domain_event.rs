//! `DomainEvent` trait — a fact that occurred in the domain.

use std::time::SystemTime;

/// A fact that occurred in the domain.
///
/// Domain events are immutable records of something that happened.
/// They carry enough information to reconstruct state when replayed in order.
pub trait DomainEvent: Send + Sync {
    /// Stable type name for this event, e.g. `"order.created"`.
    fn event_type(&self) -> &str {
        "event"
    }

    /// ID of the aggregate that produced this event.
    fn aggregate_id(&self) -> &str {
        ""
    }

    /// Wall-clock time at which the event occurred.
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}
