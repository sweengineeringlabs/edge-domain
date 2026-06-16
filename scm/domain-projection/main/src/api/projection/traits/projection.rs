use edge_domain_event::DomainEvent;

/// Consumes domain events and maintains a read model.
///
/// The read side of CQRS: apply events in stream order to build a
/// denormalised view optimised for queries.  `apply` is intentionally
/// infallible — a projection that needs to surface failures should record them
/// in the read model rather than abort the fan-out.
pub trait Projection: Send + Sync {
    /// The event type this projection handles.
    type Event: DomainEvent;

    /// The read model this projection maintains.
    type ReadModel;

    /// Apply one event to update the read model in place.
    fn apply(&mut self, event: &Self::Event);

    /// Return a reference to the current read model state.
    fn read_model(&self) -> &Self::ReadModel;
}
