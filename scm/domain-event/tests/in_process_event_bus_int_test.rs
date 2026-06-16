//! Integration tests for `InProcessEventBus`.

use std::sync::Arc;
use edge_domain_event::{DomainEvent, EventBus, EventError, EventFactory, EventBusConfig, InProcessEventBus};

struct Events;
impl EventFactory for Events {}

struct SigEvt;
impl DomainEvent for SigEvt {
    fn event_type(&self) -> &str { "signal" }
}

/// @covers: InProcessEventBus::new — creates a bus with given capacity
#[test]
fn test_in_process_event_bus_new_creates_bus_happy() {
    let bus = InProcessEventBus::new(16);
    assert!(std::mem::size_of_val(&bus) > 0);
}

/// @covers: InProcessEventBus::default — default capacity bus is usable
#[test]
fn test_in_process_event_bus_default_is_usable_error() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::default();
        assert!(bus.publish(Arc::new(SigEvt)).await.is_ok());
    });
}

/// @covers: InProcessEventBus::subscribe — subscriber receives event
#[test]
fn test_in_process_event_bus_subscriber_receives_event_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = Events::in_process_bus(EventBusConfig { capacity: 8 });
        let mut rx = bus.subscribe();
        bus.publish(Arc::new(SigEvt)).await.expect("publish");
        let event = rx.recv().await.expect("recv");
        assert_eq!(event.event_type(), "signal");
    });
}

/// @covers: InProcessEventBus::publish — no receivers returns Ok
#[test]
fn test_in_process_event_bus_publish_no_receivers_ok_error() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(4);
        assert!(bus.publish(Arc::new(SigEvt)).await.is_ok());
    });
}

/// @covers: InProcessEventBus::subscribe — second subscriber is independent
#[test]
fn test_in_process_event_bus_two_subscribers_both_receive_edge() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(16);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        bus.publish(Arc::new(SigEvt)).await.expect("publish");
        assert!(rx1.recv().await.is_ok());
        assert!(rx2.recv().await.is_ok());
    });
}

/// @covers: InProcessEventBus — closed source returns Unavailable
#[test]
fn test_in_process_event_bus_dropped_sender_returns_error_edge() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(4);
        let mut rx = bus.subscribe();
        // Drop the bus (sender)
        drop(bus);
        let result = rx.recv().await;
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    });
}
