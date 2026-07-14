#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventPublisher is exported from the crate root.

use edge_application::Domain;
use edge_application::DomainEvent;
use edge_application::DomainRuntime;
use edge_application::EventAggregateIdRequest;
use edge_application::EventAggregateIdResponse;
use edge_application::EventError;
use edge_application::EventPublisher;
use edge_application::EventPublisherPublishRequest;
use edge_application::EventTypeRequest;
use edge_application::EventTypeResponse;
use edge_application::NoopEventPublisherRequest;

struct Pulse;
impl DomainEvent for Pulse {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "pulse",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "sys",
        })
    }
}

#[tokio::test]
async fn test_event_publisher_svc_facade_noop_publish_returns_ok() {
    let p = Domain
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    assert!(p
        .publish(EventPublisherPublishRequest { event: &Pulse })
        .await
        .is_ok());
}
