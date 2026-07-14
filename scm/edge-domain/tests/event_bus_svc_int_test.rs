#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventBus is exported from the crate root.

use edge_application::Domain;
use edge_application::DomainEvent;
use edge_application::DomainRuntime;
use edge_application::EventAggregateIdRequest;
use edge_application::EventAggregateIdResponse;
use edge_application::EventBus;
use edge_application::EventBusPublishRequest;
use edge_application::EventBusSubscribeRequest;
use edge_application::EventError;
use edge_application::EventTypeRequest;
use edge_application::EventTypeResponse;
use edge_application::NoopEventBusRequest;
use std::sync::Arc;

struct Tick;
impl DomainEvent for Tick {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse { event_type: "tick" })
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
async fn test_event_bus_svc_facade_noop_publish_returns_ok() {
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    assert!(bus
        .publish(EventBusPublishRequest {
            event: Arc::new(Tick)
        })
        .await
        .is_ok());
}

#[test]
fn test_event_bus_svc_facade_subscribe_returns_receiver() {
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    let _rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
}
