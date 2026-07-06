use std::marker::PhantomData;

use edge_domain_event::DomainEvent;

use crate::api::Projection;
use crate::api::InMemoryProjection;
use crate::api::ProjectionError;
use crate::api::{ProjectionApplyRequest, ProjectionReadModelRequest, ProjectionReadModelResponse};

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

impl<E, R, F> Projection for InMemoryProjection<E, R, F>
where
    E: DomainEvent + Send + Sync,
    R: Send + Sync,
    F: Fn(&mut R, &E) + Send + Sync,
{
    type Event = E;
    type ReadModel = R;

    fn apply(&mut self, req: ProjectionApplyRequest<'_, Self::Event>) -> Result<(), ProjectionError> {
        (self.reducer)(&mut self.read_model, req.event);
        Ok(())
    }

    fn read_model(
        &self,
        _req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, Self::ReadModel>, ProjectionError> {
        Ok(ProjectionReadModelResponse { read_model: &self.read_model })
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
        let evt = InMemoryProjectionTestEvt { delta: 10 };
        p.apply(ProjectionApplyRequest { event: &evt }).expect("apply should succeed");
        assert_eq!(*p.read_model(ProjectionReadModelRequest).expect("read_model").read_model, 10);
    }

    #[test]
    fn test_apply_multiple_events_accumulates() {
        let mut p = make(0);
        let e1 = InMemoryProjectionTestEvt { delta: 3 };
        let e2 = InMemoryProjectionTestEvt { delta: 7 };
        p.apply(ProjectionApplyRequest { event: &e1 }).expect("apply should succeed");
        p.apply(ProjectionApplyRequest { event: &e2 }).expect("apply should succeed");
        assert_eq!(*p.read_model(ProjectionReadModelRequest).expect("read_model").read_model, 10);
    }

    #[test]
    fn test_read_model_initial_state_matches_seed() {
        let p = make(42);
        assert_eq!(*p.read_model(ProjectionReadModelRequest).expect("read_model").read_model, 42);
    }
}
