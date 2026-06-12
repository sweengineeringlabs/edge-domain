//! `Aggregate` trait — event-sourced aggregate root contract.

use crate::api::event::traits::DomainEvent;

/// An event-sourced aggregate root.
///
/// State is reconstructed by replaying a sequence of [`DomainEvent`] values
/// through [`Aggregate::apply`]. The initial state is produced by [`Default`].
pub trait Aggregate: Default + Send + Sync + 'static {
    /// The domain event type produced and consumed by this aggregate.
    type Event: DomainEvent + Send + Sync + Clone + 'static;

    /// Fold one event into the aggregate state.
    fn apply(&mut self, _event: &Self::Event) {}

    /// Return the stable aggregate identity string.
    fn id(&self) -> &str {
        ""
    }
}
