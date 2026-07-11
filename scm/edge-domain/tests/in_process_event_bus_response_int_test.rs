//! Integration tests for `InProcessEventBusResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Domain, DomainEvent, DomainRuntime, EventBusConfig, EventBusPublishRequest,
    EventBusSubscribeRequest, EventSourceRecvNextRequest, InProcessEventBusRequest,
};

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {}

/// @covers: InProcessEventBusResponse
#[test]
fn test_in_process_event_bus_response_subscriber_receives_published_event_happy() {
    futures::executor::block_on(async {
        let resp = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig { capacity: 8 },
            })
            .unwrap();
        let mut rx = resp
            .bus
            .subscribe(EventBusSubscribeRequest)
            .unwrap()
            .receiver;
        assert!(resp
            .bus
            .publish(EventBusPublishRequest {
                event: std::sync::Arc::new(AnyEvent)
            })
            .await
            .is_ok());
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_ok());
    });
}

/// @covers: InProcessEventBusResponse
#[test]
fn test_in_process_event_bus_response_publish_without_subscribers_returns_ok_error() {
    futures::executor::block_on(async {
        let resp = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig { capacity: 8 },
            })
            .unwrap();
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

/// @covers: InProcessEventBusResponse
#[test]
fn test_in_process_event_bus_response_bus_is_uniquely_owned_edge() {
    let resp = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig { capacity: 8 },
        })
        .unwrap();
    assert_eq!(std::sync::Arc::strong_count(&resp.bus), 1);
}
