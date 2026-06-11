//! Coverage for api/event/types/event/event_receiver.rs
use edge_domain::{Domain, DomainEvent, EventBusConfig, EventReceiver};
use futures::executor::block_on;
use std::sync::Arc;

struct TestEvent;
impl DomainEvent for TestEvent {}

#[test]
fn test_event_receiver_subscribe_returns_receiver_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let rx: EventReceiver = bus.subscribe();
        drop(rx);
    });
}

#[test]
fn test_event_receiver_recv_after_publish_returns_ok_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        assert!(bus.publish(Arc::new(TestEvent)).await.is_ok());
        assert!(rx.recv().await.is_ok());
    });
}

#[test]
fn test_event_receiver_recv_without_publish_returns_err_when_bus_dropped_edge() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        drop(bus);
        assert!(rx.recv().await.is_err());
    });
}

#[test]
fn test_event_receiver_multiple_subscribers_both_receive_event_happy() {
    block_on(async {
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        assert!(bus.publish(Arc::new(TestEvent)).await.is_ok());
        assert!(rx1.recv().await.is_ok());
        assert!(rx2.recv().await.is_ok());
    });
}
