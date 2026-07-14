//! Integration tests for `NoopEventBusResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Domain, DomainEvent, DomainRuntime, EventBusPublishRequest, EventBusSubscribeRequest,
    EventSourceRecvNextRequest, NoopEventBusRequest,
};

struct AnyEvent;
impl DomainEvent for AnyEvent {}

/// @covers: NoopEventBusResponse
#[test]
fn test_noop_event_bus_response_bus_field_publish_happy() {
    futures::executor::block_on(async {
        let resp = Domain.noop_event_bus(NoopEventBusRequest).unwrap();
        assert_eq!(
            resp.bus
                .publish(EventBusPublishRequest {
                    event: std::sync::Arc::new(AnyEvent)
                })
                .await,
            Ok(())
        );
    });
}

/// @covers: NoopEventBusResponse
#[test]
fn test_noop_event_bus_response_subscribe_receiver_immediately_closed_error() {
    futures::executor::block_on(async {
        let resp = Domain.noop_event_bus(NoopEventBusRequest).unwrap();
        let mut rx = resp
            .bus
            .subscribe(EventBusSubscribeRequest)
            .unwrap()
            .receiver;
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_err());
    });
}

/// @covers: NoopEventBusResponse
#[test]
fn test_noop_event_bus_response_bus_is_uniquely_owned_edge() {
    let resp = Domain.noop_event_bus(NoopEventBusRequest).unwrap();
    assert_eq!(std::sync::Arc::strong_count(&resp.bus), 1);
}
