//! Integration tests for `NoopEventPublisher`.

use edge_domain_event::{DomainEvent, EventFactory, EventPublisher, NoopEventPublisher};

struct Events;
impl EventFactory for Events {}

struct SomeEvt;
impl DomainEvent for SomeEvt {}

/// @covers: NoopEventPublisher::publish — always returns Ok
#[test]
fn test_noop_event_publisher_publish_returns_ok_happy() {
    let result = futures::executor::block_on(NoopEventPublisher.publish(&SomeEvt));
    assert!(result.is_ok());
}

/// @covers: NoopEventPublisher::publish — repeated calls never error
#[test]
fn test_noop_event_publisher_publish_repeated_never_errors_error() {
    for _ in 0..5 {
        assert!(futures::executor::block_on(NoopEventPublisher.publish(&SomeEvt)).is_ok());
    }
}

/// @covers: NoopEventPublisher::publish — via factory, dyn dispatch works
#[test]
fn test_noop_event_publisher_dyn_dispatch_ok_edge() {
    let pub_ = Events::noop_publisher();
    let evt: &dyn DomainEvent = &SomeEvt;
    assert!(futures::executor::block_on(pub_.publish(evt)).is_ok());
}
