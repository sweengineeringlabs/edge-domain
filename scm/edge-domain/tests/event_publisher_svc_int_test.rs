#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventPublisher is exported from the crate root.

use edge_domain::Domain;
use edge_domain::DomainEvent;
use edge_domain::EventAggregateIdRequest;
use edge_domain::EventAggregateIdResponse;
use edge_domain::EventError;
use edge_domain::EventPublisher;
use edge_domain::EventPublisherPublishRequest;
use edge_domain::EventTypeRequest;
use edge_domain::EventTypeResponse;

struct Pulse;
impl DomainEvent for Pulse {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse { event_type: "pulse" })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "sys" })
    }
}

#[tokio::test]
async fn test_event_publisher_svc_facade_noop_publish_returns_ok() {
    let p = Domain::noop_event_publisher();
    assert!(p
        .publish(EventPublisherPublishRequest { event: &Pulse })
        .await
        .is_ok());
}
