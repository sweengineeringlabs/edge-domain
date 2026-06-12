//! Integration tests for the `EventFactory` SAF facade.
#![allow(clippy::unwrap_used)]

use edge_domain::{
    ClosedEventSource, DomainEvent, EventBusConfig, EventFactory,
    InProcessEventBus, NoopEventBus, NoopEventPublisher,
};

struct TestEvents;
impl EventFactory for TestEvents {}

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self) -> &str {
        "test.any"
    }
    fn aggregate_id(&self) -> &str {
        "agg-1"
    }
    fn occurred_at(&self) -> std::time::SystemTime {
        std::time::SystemTime::now()
    }
}

// --- EventFactory::in_process_bus ---

/// @covers EventFactory::in_process_bus — happy path: returns an InProcessEventBus
#[test]
fn test_in_process_bus_returns_bus_happy() {
    let config = EventBusConfig { capacity: 256 };
    let _: InProcessEventBus = TestEvents::in_process_bus(config);
}

/// @covers EventFactory::in_process_bus — error: minimum valid capacity (1) constructs without panic
#[test]
fn test_in_process_bus_min_capacity_constructs_error() {
    let config = EventBusConfig { capacity: 1 };
    let _: InProcessEventBus = TestEvents::in_process_bus(config);
}

/// @covers EventFactory::in_process_bus — edge: successive calls produce independent buses
#[test]
fn test_in_process_bus_independent_calls_edge() {
    let _a = TestEvents::in_process_bus(EventBusConfig { capacity: 16 });
    let _b = TestEvents::in_process_bus(EventBusConfig { capacity: 32 });
}

// --- EventFactory::noop_bus ---

/// @covers EventFactory::noop_bus — happy path: returns a NoopEventBus marker
#[test]
fn test_noop_bus_returns_marker_happy() {
    let _: NoopEventBus = TestEvents::noop_bus();
}

/// @covers EventFactory::noop_bus — error: NoopEventBus is zero-size (cannot panic)
#[test]
fn test_noop_bus_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventBus>(), 0);
}

/// @covers EventFactory::noop_bus — edge: successive calls are independent
#[test]
fn test_noop_bus_independent_calls_edge() {
    let _a = TestEvents::noop_bus();
    let _b = TestEvents::noop_bus();
}

// --- EventFactory::noop_publisher ---

/// @covers EventFactory::noop_publisher — happy path: returns a NoopEventPublisher marker
#[test]
fn test_noop_publisher_returns_marker_happy() {
    let _: NoopEventPublisher = TestEvents::noop_publisher();
}

/// @covers EventFactory::noop_publisher — error: NoopEventPublisher is zero-size
#[test]
fn test_noop_publisher_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventPublisher>(), 0);
}

/// @covers EventFactory::noop_publisher — edge: successive calls are independent
#[test]
fn test_noop_publisher_independent_calls_edge() {
    let _a = TestEvents::noop_publisher();
    let _b = TestEvents::noop_publisher();
}

// --- EventFactory::in_memory_store ---

/// @covers EventFactory::in_memory_store — happy path: constructs successfully
#[test]
fn test_in_memory_store_constructs_successfully_happy() {
    let _ = TestEvents::in_memory_store::<AnyEvent>();
}

/// @covers EventFactory::in_memory_store — error: store is non-zero-size (heap-backed)
#[test]
fn test_in_memory_store_is_nonzero_size_error() {
    assert_ne!(
        std::mem::size_of_val(&TestEvents::in_memory_store::<AnyEvent>()),
        0,
    );
}

/// @covers EventFactory::in_memory_store — edge: successive calls produce independent stores
#[test]
fn test_in_memory_store_successive_calls_independent_edge() {
    let _a = TestEvents::in_memory_store::<AnyEvent>();
    let _b = TestEvents::in_memory_store::<AnyEvent>();
}

// --- EventFactory::closed_source ---

/// @covers EventFactory::closed_source — happy path: returns a ClosedEventSource marker
#[test]
fn test_closed_source_returns_marker_happy() {
    let _: ClosedEventSource = TestEvents::closed_source();
}

/// @covers EventFactory::closed_source — error: ClosedEventSource is zero-size
#[test]
fn test_closed_source_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<ClosedEventSource>(), 0);
}

/// @covers EventFactory::closed_source — edge: successive calls produce independent markers
#[test]
fn test_closed_source_successive_calls_independent_edge() {
    let _a = TestEvents::closed_source();
    let _b = TestEvents::closed_source();
}
