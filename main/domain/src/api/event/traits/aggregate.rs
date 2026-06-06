//! `Aggregate` — state rebuilt by applying a sequence of domain events.

use crate::api::event::DomainEvent;

/// An aggregate whose state is derived entirely from a sequence of domain events.
///
/// Consumers implement this trait on their state type.  The
/// [`Domain::reconstitute`](crate::Domain::reconstitute) SAF helper loads all
/// events from an [`crate::EventStore`] and calls
/// [`apply`](Aggregate::apply) in sequence to rebuild the aggregate.
///
/// ```rust,ignore
/// #[derive(Default)]
/// struct Counter { id: String, value: u64 }
///
/// impl Aggregate for Counter {
///     type Event = CounterIncremented;
///
///     fn apply(&mut self, event: &CounterIncremented) {
///         self.id    = event.counter_id.clone();
///         self.value += 1;
///     }
///
///     fn id(&self) -> &str { &self.id }
/// }
/// ```
pub trait Aggregate: Default + Send + Sync + 'static {
    /// The domain event type this aggregate is sourced from.
    type Event: DomainEvent + Send + Sync + Clone + 'static;

    /// Apply a single event to mutate the aggregate's state.
    ///
    /// Called in version order during reconstitution.  Must be deterministic —
    /// the same sequence of events must always produce the same state.
    fn apply(&mut self, _event: &Self::Event) {}

    /// Return the aggregate's identity.
    fn id(&self) -> &str {
        ""
    }
}
