use edge_domain_event::DomainEvent;

use crate::api::Projection;
use crate::api::InMemoryProjection;

impl<E, R, F> Projection for InMemoryProjection<E, R, F>
where
    E: DomainEvent + Send + Sync,
    R: Send + Sync,
    F: Fn(&mut R, &E) + Send + Sync,
{
    type Event = E;
    type ReadModel = R;

    fn apply(&mut self, event: &Self::Event) {
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
        delta: i64,
    }

    impl DomainEvent for InMemoryProjectionTestEvt {
        fn aggregate_id(&self, _req: edge_domain_event::EventAggregateIdRequest) -> Result<edge_domain_event::EventAggregateIdResponse<'_>, edge_domain_event::EventError> {
            Ok(edge_domain_event::EventAggregateIdResponse { aggregate_id: "test" })
        }
        fn occurred_at(&self, _req: edge_domain_event::EventOccurredAtRequest) -> Result<edge_domain_event::EventOccurredAtResponse, edge_domain_event::EventError> {
            Ok(edge_domain_event::EventOccurredAtResponse { occurred_at: SystemTime::UNIX_EPOCH })
        }
    }

    fn make(initial: i64) -> InMemoryProjection<InMemoryProjectionTestEvt, i64, impl Fn(&mut i64, &InMemoryProjectionTestEvt) + Send + Sync> {
        InMemoryProjection::new(initial, |total: &mut i64, e: &InMemoryProjectionTestEvt| {
            *total += e.delta;
        })
    }

    #[test]
    fn test_apply_single_event_updates_read_model() {
        let mut p = make(0);
        p.apply(&InMemoryProjectionTestEvt { delta: 10 });
        assert_eq!(*p.read_model(), 10);
    }

    #[test]
    fn test_apply_multiple_events_accumulates() {
        let mut p = make(0);
        p.apply(&InMemoryProjectionTestEvt { delta: 3 });
        p.apply(&InMemoryProjectionTestEvt { delta: 7 });
        assert_eq!(*p.read_model(), 10);
    }

    #[test]
    fn test_read_model_initial_state_matches_seed() {
        let p = make(42);
        assert_eq!(*p.read_model(), 42);
    }
}
