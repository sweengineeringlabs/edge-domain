//! Integration tests for `EventReceiver`.

use std::sync::Arc;
use edge_domain_event::{DomainEvent, EventError, EventFactory, EventBus, EventBusConfig};

struct Events;
impl EventFactory for Events {}

struct PingEvt;
impl DomainEvent for PingEvt {
    fn event_type(&self) -> &str { "ping" }
}

/// @covers: EventReceiver::recv — receives event published to bus
#[test]
fn test_event_receiver_recv_returns_published_event_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = Events::in_process_bus(EventBusConfig { capacity: 4 });
        let mut rx = bus.subscribe();
        bus.publish(Arc::new(PingEvt)).await.expect("publish");
        let event = rx.recv().await.expect("recv");
        assert_eq!(event.event_type(), "ping");
    });
}

/// @covers: EventReceiver::recv — closed bus returns Unavailable error
#[test]
fn test_event_receiver_recv_on_closed_bus_returns_unavailable_error() {
    use edge_domain_event::InProcessEventBus;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(4);
        let mut rx = bus.subscribe();
        drop(bus);
        let result = rx.recv().await;
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    });
}

/// @covers: EventReceiver::recv — noop bus receiver returns Unavailable immediately
#[test]
fn test_event_receiver_recv_noop_bus_returns_unavailable_edge() {
    use edge_domain_event::EventBus;
    use edge_domain_event::NoopEventBus;
    let mut rx = NoopEventBus.subscribe();
    let result = futures::executor::block_on(rx.recv());
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}
