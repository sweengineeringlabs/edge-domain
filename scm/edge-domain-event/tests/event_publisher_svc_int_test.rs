//! SAF facade tests — `EventPublisher` trait via `NoopEventPublisher`.

use edge_domain_event::{DomainEvent, EventFactory, EventPublisher, NoopEventPublisher};

struct Events;
impl EventFactory for Events {}

struct Evt;
impl DomainEvent for Evt {}

/// @covers: NoopEventPublisher::publish — returns Ok for any event
#[test]
fn test_publish_noop_publisher_returns_ok_happy() {
    let result = futures::executor::block_on(NoopEventPublisher.publish(&Evt));
    assert!(result.is_ok());
}

/// @covers: NoopEventPublisher — constructed via EventFactory
#[test]
fn test_noop_publisher_via_factory_returns_ok_happy() {
    let pub_ = Events::noop_publisher();
    let result = futures::executor::block_on(pub_.publish(&Evt));
    assert!(result.is_ok());
}

/// @covers: NoopEventPublisher::publish — dyn dispatch works
#[test]
fn test_publish_dyn_dispatch_returns_ok_edge() {
    let pub_: &dyn EventPublisher = &NoopEventPublisher;
    let evt: &dyn DomainEvent = &Evt;
    assert!(futures::executor::block_on(pub_.publish(evt)).is_ok());
}

/// @covers: NoopEventPublisher::publish — repeated calls never error
#[test]
fn test_publish_repeated_calls_never_error_error() {
    for _ in 0..5 {
        let result = futures::executor::block_on(NoopEventPublisher.publish(&Evt));
        assert!(result.is_ok(), "noop publisher must never return error");
    }
}
