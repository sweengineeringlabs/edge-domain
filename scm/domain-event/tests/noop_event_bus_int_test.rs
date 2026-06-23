//! Integration tests for `NoopEventBus`.

use std::sync::Arc;
use edge_domain_event::{DomainEvent, EventBus, EventError, EventBootstrap, NoopEventBus};

struct Events;
impl EventBootstrap for Events {}

struct SignalEvt;
impl DomainEvent for SignalEvt {}

/// @covers: NoopEventBus::publish — always returns Ok
#[test]
fn test_noop_event_bus_publish_returns_ok_happy() {
    let result = futures::executor::block_on(NoopEventBus.publish(Arc::new(SignalEvt)));
    assert_eq!(result, Ok(()), "noop bus publish must always return Ok(())");
}

/// @covers: NoopEventBus::publish — repeated publishes never error
#[test]
fn test_noop_event_bus_publish_repeated_never_errors_error() {
    for _ in 0..5 {
        let result = futures::executor::block_on(NoopEventBus.publish(Arc::new(SignalEvt)));
        assert_eq!(result, Ok(()));
    }
}

/// @covers: NoopEventBus::subscribe — subscribe returns immediately-closed receiver
#[test]
fn test_noop_event_bus_subscribe_returns_closed_receiver_edge() {
    let mut rx = Events::noop_bus().subscribe();
    let result = futures::executor::block_on(rx.recv());
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}
