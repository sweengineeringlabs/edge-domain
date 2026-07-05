//! Integration tests for `StdEventFactory`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_event::{
    AggregateIdentityRequest, Aggregate, DomainEvent, EventBus, EventBusConfig,
    EventBusPublishRequest, EventBootstrap, EventPublisher, EventPublisherPublishRequest,
    EventSource, EventSourceRecvNextRequest, EventStore, EventStoreAppendRequest, EventStoreError,
    EventTypeRequest, ExpectedVersion, NoopDomainEvent, StdEventFactory,
};

/// @covers: StdEventFactory::noop_bus — returns a zero-sized NoopEventBus
#[test]
fn test_noop_bus_std_event_factory_is_zero_sized_happy() {
    let bus = StdEventFactory::noop_bus();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: StdEventFactory::noop_publisher — returns a zero-sized NoopEventPublisher
#[test]
fn test_noop_publisher_std_event_factory_is_zero_sized_happy() {
    let pub_ = StdEventFactory::noop_publisher();
    assert_eq!(std::mem::size_of_val(&pub_), 0);
}

/// @covers: StdEventFactory::closed_source — returns a zero-sized ClosedEventSource
#[test]
fn test_closed_source_std_event_factory_is_zero_sized_happy() {
    let src = StdEventFactory::closed_source();
    assert_eq!(std::mem::size_of_val(&src), 0);
}

/// @covers: StdEventFactory::in_memory_store — append succeeds on empty store
#[test]
fn test_in_memory_store_std_event_factory_append_succeeds_happy() {
    let store = StdEventFactory::in_memory_store::<NoopDomainEvent>();
    let resp = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-1",
        events: vec![NoopDomainEvent],
        expected: ExpectedVersion::Any,
    }))
    .expect("append");
    assert_eq!(resp.sequence, 1);
}

/// @covers: StdEventFactory::in_memory_store — conflict on NoStream after stream exists
#[test]
fn test_in_memory_store_std_event_factory_conflict_on_double_no_stream_error() {
    let store = StdEventFactory::in_memory_store::<NoopDomainEvent>();
    futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-2",
        events: vec![NoopDomainEvent],
        expected: ExpectedVersion::NoStream,
    }))
    .expect("first append");
    let err = futures::executor::block_on(store.append(EventStoreAppendRequest {
        aggregate_id: "agg-2",
        events: vec![NoopDomainEvent],
        expected: ExpectedVersion::NoStream,
    }))
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: StdEventFactory::in_process_bus — constructed with custom capacity
#[test]
fn test_in_process_bus_std_event_factory_custom_capacity_edge() {
    let _bus = StdEventFactory::in_process_bus(EventBusConfig { capacity: 64 });
    // Verify the bus is a non-zero-sized in-process implementation
    assert!(std::mem::size_of_val(&_bus) > 0);
}

/// @covers: StdEventFactory::noop_bus — publish never errors
#[test]
fn test_noop_bus_publish_never_errors_error() {
    use std::sync::Arc;
    let bus = StdEventFactory::noop_bus();
    let result = futures::executor::block_on(
        bus.publish(EventBusPublishRequest { event: Arc::new(NoopDomainEvent) }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: StdEventFactory::noop_publisher — publish never errors
#[test]
fn test_noop_publisher_publish_never_errors_error() {
    let pub_ = StdEventFactory::noop_publisher();
    let result = futures::executor::block_on(
        pub_.publish(EventPublisherPublishRequest { event: &NoopDomainEvent }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: StdEventFactory::closed_source — recv_next always returns Unavailable
#[test]
fn test_closed_source_recv_next_returns_unavailable_error() {
    use edge_domain_event::EventError;
    let mut src = StdEventFactory::closed_source();
    let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: StdEventFactory::std — constructs the factory itself
#[test]
fn test_std_factory_constructs_std_event_factory_happy() {
    let _f = StdEventFactory::std();
    // Verify the factory is a zero-sized marker type
    assert_eq!(std::mem::size_of_val(&_f), 0);
}

/// @covers: StdEventFactory::noop_event — returns a NoopDomainEvent
#[test]
fn test_noop_event_returns_noop_domain_event_happy() {
    let evt = StdEventFactory::noop_event();
    assert_eq!(evt.event_type(EventTypeRequest).unwrap().event_type, "event");
}

/// @covers: StdEventFactory::noop_aggregate — returns a NoopAggregate
#[test]
fn test_noop_aggregate_returns_noop_aggregate_happy() {
    let agg = StdEventFactory::noop_aggregate();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}
