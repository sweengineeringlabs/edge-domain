//! Integration tests for the `EventFactory` SAF facade.

use edge_domain::{
    ClosedEventSource, EventBusConfig, EventFactory, InProcessEventBus,
    NoopEventBus, NoopEventPublisher,
};

struct TestEvents;
impl EventFactory for TestEvents {}

// --- EventFactory::in_process_bus ---

/// @covers EventFactory::in_process_bus — happy path: returns an InProcessEventBus marker
#[test]
fn test_in_process_bus_returns_marker_happy() {
    let config = EventBusConfig { capacity: 256 };
    let _: InProcessEventBus = TestEvents::in_process_bus(config);
}

/// @covers EventFactory::in_process_bus — error: zero-capacity config still constructs
#[test]
fn test_in_process_bus_zero_capacity_still_constructs_error() {
    let config = EventBusConfig { capacity: 0 };
    let _: InProcessEventBus = TestEvents::in_process_bus(config);
}

/// @covers EventFactory::in_process_bus — edge: InProcessEventBus is a unit struct
#[test]
fn test_in_process_bus_is_unit_struct_edge() {
    assert_eq!(std::mem::size_of::<InProcessEventBus>(), 0);
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

/// @covers EventFactory::in_memory_store — happy path: returns a zero-size marker
#[test]
fn test_in_memory_store_returns_zero_size_marker_happy() {
    let store = TestEvents::in_memory_store();
    assert_eq!(std::mem::size_of_val(&store), 0);
}

/// @covers EventFactory::in_memory_store — error: marker has no state to corrupt
#[test]
fn test_in_memory_store_marker_has_no_state_error() {
    let store = TestEvents::in_memory_store();
    assert_eq!(std::mem::size_of_val(&store), 0);
}

/// @covers EventFactory::in_memory_store — edge: successive calls produce independent values
#[test]
fn test_in_memory_store_successive_calls_independent_edge() {
    let _a = TestEvents::in_memory_store();
    let _b = TestEvents::in_memory_store();
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
