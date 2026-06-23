//! Integration tests for `StdEventFactory`.

use edge_domain_event::{
    Aggregate, DomainEvent, EventBus, EventBusConfig, EventBootstrap, EventPublisher, EventSource,
    EventStore, EventStoreError, ExpectedVersion, NoopDomainEvent, StdEventFactory,
};

/// @covers: StdEventFactory — noop_bus returns a zero-sized type
#[test]
fn test_noop_bus_std_event_factory_is_zero_sized_happy() {
    let bus = StdEventFactory::noop_bus();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: StdEventFactory — noop_publisher returns a zero-sized type
#[test]
fn test_noop_publisher_std_event_factory_is_zero_sized_happy() {
    let pub_ = StdEventFactory::noop_publisher();
    assert_eq!(std::mem::size_of_val(&pub_), 0);
}

/// @covers: StdEventFactory — closed_source is zero-sized
#[test]
fn test_closed_source_std_event_factory_is_zero_sized_happy() {
    let src = StdEventFactory::closed_source();
    assert_eq!(std::mem::size_of_val(&src), 0);
}

/// @covers: StdEventFactory — in_memory_store append succeeds
#[test]
fn test_in_memory_store_std_event_factory_append_returns_sequence_happy() {
    let store = StdEventFactory::in_memory_store::<NoopDomainEvent>();
    let seq = futures::executor::block_on(
        store.append("agg-1", vec![NoopDomainEvent], ExpectedVersion::Any),
    )
    .expect("append");
    assert_eq!(seq, 1);
}

/// @covers: StdEventFactory — in_memory_store NoStream conflict
#[test]
fn test_in_memory_store_std_event_factory_no_stream_conflict_error() {
    let store = StdEventFactory::in_memory_store::<NoopDomainEvent>();
    futures::executor::block_on(
        store.append("agg-2", vec![NoopDomainEvent], ExpectedVersion::NoStream),
    )
    .expect("first");
    let err = futures::executor::block_on(
        store.append("agg-2", vec![NoopDomainEvent], ExpectedVersion::NoStream),
    )
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: StdEventFactory — in_process_bus constructed with non-default capacity
#[test]
fn test_in_process_bus_std_event_factory_large_capacity_edge() {
    let _bus = StdEventFactory::in_process_bus(EventBusConfig { capacity: 256 });
    // Verify bus is non-zero-sized (it holds channels)
    assert!(std::mem::size_of_val(&_bus) > 0);
}

/// @covers: StdEventFactory — noop_bus publish never errors
#[test]
fn test_noop_bus_publish_returns_ok_error() {
    use std::sync::Arc;
    let bus = StdEventFactory::noop_bus();
    let result = futures::executor::block_on(bus.publish(Arc::new(NoopDomainEvent)));
    assert!(result.is_ok());
}

/// @covers: StdEventFactory — noop_publisher publish never errors
#[test]
fn test_noop_publisher_publish_returns_ok_error() {
    let pub_ = StdEventFactory::noop_publisher();
    let result = futures::executor::block_on(pub_.publish(&NoopDomainEvent));
    assert!(result.is_ok());
}

/// @covers: StdEventFactory — closed_source returns Unavailable
#[test]
fn test_closed_source_recv_next_returns_unavailable_edge() {
    use edge_domain_event::EventError;
    let mut src = StdEventFactory::closed_source();
    let result = futures::executor::block_on(src.recv_next());
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}
