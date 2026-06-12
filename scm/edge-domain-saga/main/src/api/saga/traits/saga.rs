use std::hash::Hash;

use edge_domain_command::Command;
use edge_domain_event::DomainEvent;

/// Orchestrates a long-running business process.
///
/// A saga reacts to [`DomainEvent`]s, maintains its own durable state, and
/// emits [`Command`]s to drive the process toward completion or to compensate
/// on failure.  Dispatching the staged commands is the caller's responsibility.
pub trait Saga: Send + Sync {
    /// Unique identifier for this saga instance.
    type SagaId: Eq + Hash + Clone + Send + Sync;

    /// The union of events this saga handles.
    type Event: DomainEvent;

    /// Commands this saga can emit.
    type Command: Command;

    /// Apply an event; return zero or more commands to dispatch.
    fn handle(&mut self, event: &Self::Event) -> Vec<Self::Command>;

    /// Whether this saga has reached a terminal state (completed or compensated).
    fn is_complete(&self) -> bool;
}
