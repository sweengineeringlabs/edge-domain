#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — DomainEvent trait is exported from the crate root.

use edge_domain::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventTypeRequest, EventTypeResponse,
};

struct OrderPlaced {
    id: String,
}
impl DomainEvent for OrderPlaced {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "order.placed",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
}

#[test]
fn test_domain_event_svc_facade_event_type_returns_string() {
    let evt = OrderPlaced { id: "x".into() };
    assert_eq!(
        evt.event_type(EventTypeRequest).unwrap().event_type,
        "order.placed"
    );
}

#[test]
fn test_domain_event_svc_facade_aggregate_id_returns_value() {
    let evt = OrderPlaced {
        id: "agg-42".into(),
    };
    assert_eq!(
        evt.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "agg-42"
    );
}
