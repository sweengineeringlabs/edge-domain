//! In-memory projection — folds events into a read model via a reducer.
//!
//! A general-purpose reference [`Projection`] for development and testing.
//! State lives in process memory and is lost when the process stops.

use std::marker::PhantomData;

use crate::api::event::DomainEvent;
use crate::api::event::EventEnvelope;
use crate::api::projection::Projection;

/// Builds a read model `R` by folding each event through a reducer `F`.
///
/// The `PhantomData<fn(&E)>` marker binds the event type without requiring
/// `E: Send + Sync` on the struct itself — those bounds are demanded only where
/// the [`Projection`] impl needs them.
pub(crate) struct InMemoryProjection<E, R, F> {
    read_model: R,
    reducer: F,
    _event: PhantomData<fn(&E)>,
}

impl<E, R, F> InMemoryProjection<E, R, F> {
    /// Construct a projection seeded with `initial` and updated by `reducer`.
    pub(crate) fn new(initial: R, reducer: F) -> Self {
        Self {
            read_model: initial,
            reducer,
            _event: PhantomData,
        }
    }
}

impl<E, R, F> Projection for InMemoryProjection<E, R, F>
where
    E: DomainEvent + Send + Sync,
    R: Send + Sync,
    F: Fn(&mut R, &EventEnvelope<E>) + Send + Sync,
{
    type Event = E;
    type ReadModel = R;

    fn apply(&mut self, event: &EventEnvelope<Self::Event>) {
        (self.reducer)(&mut self.read_model, event);
    }

    fn read_model(&self) -> &Self::ReadModel {
        &self.read_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[derive(Clone)]
    struct InMemoryProjectionTestEvt {
        id: String,
        amount: u64,
    }

    impl DomainEvent for InMemoryProjectionTestEvt {
        fn event_type(&self) -> &str {
            "test.credited"
        }
        fn aggregate_id(&self) -> &str {
            &self.id
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    fn envelope(seq: u64, amount: u64) -> EventEnvelope<InMemoryProjectionTestEvt> {
        EventEnvelope {
            aggregate_id: "acct-1".to_string(),
            sequence: seq,
            occurred_at: SystemTime::now(),
            event: InMemoryProjectionTestEvt {
                id: "acct-1".to_string(),
                amount,
            },
        }
    }

    #[test]
    fn test_new_seeds_read_model_with_initial_value() {
        let p = InMemoryProjection::<InMemoryProjectionTestEvt, u64, _>::new(
            7u64,
            |total: &mut u64, e: &EventEnvelope<InMemoryProjectionTestEvt>| {
                *total += e.event.amount;
            },
        );
        assert_eq!(*p.read_model(), 7);
    }

    #[test]
    fn test_apply_folds_event_into_read_model() {
        let mut p = InMemoryProjection::new(
            0u64,
            |total: &mut u64, e: &EventEnvelope<InMemoryProjectionTestEvt>| {
                *total += e.event.amount;
            },
        );
        p.apply(&envelope(1, 10));
        p.apply(&envelope(2, 32));
        assert_eq!(*p.read_model(), 42);
    }

    #[test]
    fn test_apply_no_events_leaves_initial_read_model_unchanged() {
        let p = InMemoryProjection::<InMemoryProjectionTestEvt, u64, _>::new(
            5u64,
            |total: &mut u64, e: &EventEnvelope<InMemoryProjectionTestEvt>| {
                *total += e.event.amount;
            },
        );
        assert_eq!(*p.read_model(), 5);
    }
}
