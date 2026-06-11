//! `DomainEvent` trait — immutable record of something that happened in the domain.

use std::time::SystemTime;

/// An immutable record of something that happened within the domain.
///
/// Events are facts — they describe what happened, not what should happen.
/// Implementations must be `Send + Sync` and carry no mutable state after
/// construction.
///
/// `edge-domain` ships pipeline stage lifecycle events implementing this trait:
/// [`crate::api::event::types::stage_started::StageStarted`],
/// [`crate::api::event::types::stage_completed::StageCompleted`] (built with
/// [`crate::api::event::types::stage_completed_builder::StageCompletedBuilder`]),
/// [`crate::api::event::types::stage_failed::StageFailed`] (built with
/// [`crate::api::event::types::stage_failed_builder::StageFailedBuilder`]), and
/// [`crate::api::event::types::stage_skipped::StageSkipped`].
///
/// ```rust,ignore
/// struct OrderCreated { order_id: String, occurred_at: SystemTime }
///
/// impl DomainEvent for OrderCreated {
///     fn event_type(&self)   -> &str        { "order.created" }
///     fn aggregate_id(&self) -> &str        { &self.order_id }
///     fn occurred_at(&self)  -> SystemTime  { self.occurred_at }
/// }
/// ```
pub trait DomainEvent: Send + Sync {
    /// Fully-qualified event type name (e.g. `"order.created"`).
    fn event_type(&self) -> &str {
        "event"
    }

    /// Identity of the aggregate that produced this event.
    fn aggregate_id(&self) -> &str {
        ""
    }

    /// Wall-clock time at which the event occurred.
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}
