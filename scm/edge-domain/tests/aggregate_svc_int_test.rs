#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Aggregate trait is exported from the crate root.

use edge_domain::Aggregate;
use edge_domain::AggregateApplyRequest;
use edge_domain::AggregateApplyResponse;
use edge_domain::DomainEvent;
use edge_domain::EventAggregateIdRequest;
use edge_domain::EventAggregateIdResponse;
use edge_domain::EventError;
use edge_domain::EventTypeRequest;
use edge_domain::EventTypeResponse;

#[derive(Clone)]
struct TestDomainEvent;
impl DomainEvent for TestDomainEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.event",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "agg-1",
        })
    }
}

#[derive(Default)]
struct TestAggregate {
    applied: u32,
}
impl Aggregate for TestAggregate {
    type Event = TestDomainEvent;
    fn apply(
        &mut self,
        _req: AggregateApplyRequest<'_, Self::Event>,
    ) -> Result<AggregateApplyResponse, EventError> {
        self.applied += 1;
        Ok(AggregateApplyResponse)
    }
}

#[test]
fn test_aggregate_svc_facade_apply_increments_count() {
    let mut agg = TestAggregate::default();
    agg.apply(AggregateApplyRequest {
        event: &TestDomainEvent,
    })
    .unwrap();
    assert_eq!(agg.applied, 1);
}
