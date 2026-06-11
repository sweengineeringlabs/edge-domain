//! `Saga` — orchestrates a long-running business process.

use std::hash::Hash;

use crate::api::command::Command;
use crate::api::event::DomainEvent;
use crate::api::event::EventEnvelope;

/// Orchestrates a long-running business process (a.k.a. ProcessManager).
///
/// A saga reacts to [`DomainEvent`]s, maintains its own durable state, and
/// emits [`Command`]s to drive the process toward completion or to compensate
/// on failure.
///
/// # Shared ownership
///
/// [`handle`](Saga::handle) takes `&mut self` — sagas are stateful.  Callers
/// that share a saga across tasks or threads must wrap it in a `Mutex` (or
/// equivalent); the trait deliberately does not impose interior mutability.
///
/// # Dispatch is the caller's job
///
/// `handle` is synchronous: it applies the event to the saga's state and
/// *stages* commands by returning them.  Actually dispatching those commands
/// through a [`CommandBus`](crate::CommandBus) — including the async middleware
/// stack — is the caller's responsibility, not the saga's.
pub trait Saga: Send + Sync {
    /// Unique identifier for this saga instance.
    type SagaId: Eq + Hash + Clone + Send + Sync;

    /// The union of events this saga handles.
    type Event: DomainEvent;

    /// Commands this saga can emit.
    type Command: Command;

    /// Handle an incoming event; return zero or more commands to dispatch.
    ///
    /// Returning an empty `Vec` is valid — it means the event advanced (or did
    /// not affect) the saga's state without staging new work.
    fn handle(&mut self, event: &EventEnvelope<Self::Event>) -> Vec<Self::Command>;

    /// Whether this saga has reached a terminal state.
    ///
    /// Terminal covers both successful completion and compensation (rollback).
    fn is_complete(&self) -> bool;
}
