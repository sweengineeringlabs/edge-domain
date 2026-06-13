use edge_domain_event::DomainEvent;

use crate::api::projection::errors::ProjectionError;
use crate::api::projection::types::{InMemoryProjection, StdProjectionFactory};

/// Factory for creating and driving [`InMemoryProjection`] instances.
pub trait ProjectionFactory {
    /// Construct an in-memory projection seeded with `initial`, updated by `reducer`.
    fn in_memory<E, R, F>(initial: R, reducer: F) -> InMemoryProjection<E, R, F>
    where
        E: DomainEvent + Send + Sync,
        R: Send + Sync,
        F: Fn(&mut R, &E) + Send + Sync,
    {
        InMemoryProjection::new(initial, reducer)
    }

    /// Feed a slice of events into a projection; returns the count applied.
    ///
    /// Returns [`ProjectionError::EmptyStream`] when `events` is empty.
    fn try_drain<E, R, F>(
        projection: &mut InMemoryProjection<E, R, F>,
        events: &[E],
    ) -> Result<usize, ProjectionError>
    where
        E: DomainEvent + Send + Sync,
        R: Send + Sync,
        F: Fn(&mut R, &E) + Send + Sync,
    {
        if events.is_empty() {
            return Err(ProjectionError::EmptyStream);
        }
        for e in events {
            (projection.reducer)(&mut projection.read_model, e);
        }
        Ok(events.len())
    }

    /// Return the standard projection-factory instance.
    fn std_factory() -> StdProjectionFactory {
        StdProjectionFactory
    }
}
