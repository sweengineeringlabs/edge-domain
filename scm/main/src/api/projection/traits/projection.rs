//! `Projection` — the read side of CQRS.

use crate::api::event::DomainEvent;
use crate::api::event::EventEnvelope;

/// Consumes domain events and maintains a read model.
///
/// A projection is the read side of CQRS: it listens to [`DomainEvent`]s and
/// builds/updates a denormalised view optimised for queries.  The command side
/// ([`Aggregate`](crate::Aggregate), [`EventStore`](crate::EventStore)) is the
/// source of truth; a projection derives a query-optimised view from it.
///
/// # Shared ownership
///
/// [`apply`](Projection::apply) takes `&mut self`.  Callers that need to share
/// a projection across tasks or threads must wrap it in a `Mutex` (or
/// equivalent) — the trait deliberately does not impose interior mutability so
/// single-owner consumers pay no synchronisation cost.
///
/// # Fallibility
///
/// Neither method returns `Result`.  Applying an event to a read model is
/// modelled as infallible; a projection that needs to surface failures should
/// record them in its read model rather than abort the fan-out.
pub trait Projection: Send + Sync {
    /// The event type this projection handles.
    type Event: DomainEvent;

    /// The read model this projection maintains.
    type ReadModel;

    /// Apply an event to update the read model.
    ///
    /// Called once per event in stream order.  Must be deterministic — the same
    /// sequence of events must always produce the same read model.
    fn apply(&mut self, event: &EventEnvelope<Self::Event>);

    /// Return the current read model state.
    fn read_model(&self) -> &Self::ReadModel;
}
