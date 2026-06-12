use std::marker::PhantomData;

use edge_domain_event::DomainEvent;

/// Builds a read model `R` by folding each event through a reducer `F`.
///
/// The `PhantomData<fn(&E)>` marker binds the event type without requiring
/// `E: Send + Sync` on the struct itself — those bounds are demanded only where
/// the [`Projection`](crate::Projection) impl needs them.
pub struct InMemoryProjection<E, R, F> {
    pub(crate) read_model: R,
    pub(crate) reducer: F,
    _event: PhantomData<fn(&E)>,
}

impl<E, R, F> InMemoryProjection<E, R, F>
where
    E: DomainEvent,
    F: Fn(&mut R, &E),
{
    /// Construct a projection seeded with `initial`, updated by `reducer`.
    pub fn new(initial: R, reducer: F) -> Self {
        Self {
            read_model: initial,
            reducer,
            _event: PhantomData,
        }
    }
}
