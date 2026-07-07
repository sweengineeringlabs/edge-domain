//! Coverage for `EventSource::recv` (api/event/traits/event_source.rs)
#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_domain::{
    Domain, DomainEvent, EventBusConfig, EventBusPublishRequest, EventBusSubscribeRequest,
    EventSource, EventSourceRecvNextRequest,
};
use futures::executor::block_on;
use std::sync::Arc;

struct TestEvent;
impl DomainEvent for TestEvent {}

#[test]
fn test_event_receiver_subscribe_returns_receiver_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let rx: Box<dyn EventSource> = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        drop(rx);
    });
}

#[test]
fn test_event_receiver_recv_after_publish_returns_ok_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(TestEvent)
            })
            .await
            .is_ok(),
            "publish should succeed"
        );
        assert!(
            rx.recv_next(EventSourceRecvNextRequest).await.is_ok(),
            "receiver should get event"
        );
    });
}

#[test]
fn test_event_receiver_recv_without_publish_returns_err_when_bus_dropped_edge() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        drop(bus);
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_err());
    });
}

#[test]
fn test_event_receiver_multiple_subscribers_both_receive_event_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx1 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let mut rx2 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(TestEvent)
            })
            .await
            .is_ok(),
            "publish should succeed"
        );
        assert!(
            rx1.recv_next(EventSourceRecvNextRequest).await.is_ok(),
            "first receiver should get event"
        );
        assert!(
            rx2.recv_next(EventSourceRecvNextRequest).await.is_ok(),
            "second receiver should get event"
        );
    });
}
