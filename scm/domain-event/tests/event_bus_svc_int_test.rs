//! SAF facade tests — `EventBus` trait via `NoopEventBus` and `InProcessEventBus`.

use std::sync::Arc;
use edge_domain_event::{DomainEvent, EventBus, EventError, EventBootstrap, NoopEventBus};

struct Events;
impl EventBootstrap for Events {}

struct Evt;
impl DomainEvent for Evt {
    fn event_type(&self) -> &str { "evt" }
}

/// @covers: NoopEventBus::publish — always Ok
#[test]
fn test_publish_noop_returns_ok_happy() {
    let result = futures::executor::block_on(NoopEventBus.publish(Arc::new(Evt)));
    assert!(result.is_ok());
}

/// @covers: NoopEventBus::subscribe — receiver is immediately closed
#[test]
fn test_subscribe_noop_receiver_unavailable_error() {
    let mut rx = NoopEventBus.subscribe();
    let result = futures::executor::block_on(rx.recv());
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: InProcessEventBus::publish — succeeds with subscriber
#[test]
fn test_in_process_bus_publish_with_subscriber_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = Events::in_process_bus(edge_domain_event::EventBusConfig { capacity: 8 });
        let mut rx = bus.subscribe();
        bus.publish(Arc::new(Evt)).await.expect("publish");
        let e = rx.recv().await.expect("recv");
        assert_eq!(e.event_type(), "evt");
    });
}

/// @covers: InProcessEventBus::publish — no subscribers still returns Ok
#[test]
fn test_in_process_bus_publish_no_subscribers_returns_ok_edge() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = Events::in_process_bus(edge_domain_event::EventBusConfig::default());
        assert!(bus.publish(Arc::new(Evt)).await.is_ok());
    });
}

/// @covers: NoopEventBus::publish — noop bus never errors even for repeated publishes
#[test]
fn test_publish_noop_repeated_publishes_all_ok_error() {
    for _ in 0..3 {
        let result = futures::executor::block_on(NoopEventBus.publish(Arc::new(Evt)));
        assert!(result.is_ok(), "noop publish must never return error");
    }
}

/// @covers: NoopEventBus::subscribe — subscribe returns a receiver
#[test]
fn test_subscribe_noop_returns_receiver_happy() {
    let _rx = NoopEventBus.subscribe();
    // subscribe must not panic
}

/// @covers: NoopEventBus::subscribe — multiple subscribe calls each get a closed receiver
#[test]
fn test_subscribe_noop_multiple_calls_all_closed_edge() {
    for _ in 0..3 {
        let mut rx = NoopEventBus.subscribe();
        assert!(matches!(
            futures::executor::block_on(rx.recv()),
            Err(EventError::Unavailable(_))
        ));
    }
}
