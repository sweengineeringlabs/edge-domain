//! `DomainEvent` trait — immutable record of something that happened in the domain.

use std::time::SystemTime;

/// An immutable record of something that happened within the domain.
///
/// Events are facts — they describe what happened, not what should happen.
/// Implementations must be `Send + Sync` and carry no mutable state after
/// construction.
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
    fn event_type(&self) -> &str;

    /// Identity of the aggregate that produced this event.
    fn aggregate_id(&self) -> &str;

    /// Wall-clock time at which the event occurred.
    fn occurred_at(&self) -> SystemTime;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_event_is_object_safe() {
        fn _assert(_: &dyn DomainEvent) {}
    }
}
