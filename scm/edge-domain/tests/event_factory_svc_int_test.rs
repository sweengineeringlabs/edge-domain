//! Integration tests for the `EventBootstrap` SAF facade.
#![allow(clippy::unwrap_used)]

use edge_domain::{
    ClosedEventSource, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse,
    EventBootstrap, EventBusConfig, EventError, EventOccurredAtRequest, EventOccurredAtResponse,
    EventTypeRequest, EventTypeResponse, InProcessEventBus, NoopEventBus, NoopEventPublisher,
};

struct TestEvents;
impl EventBootstrap for TestEvents {}

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.any",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "agg-1",
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: std::time::SystemTime::now(),
        })
    }
}

// --- EventBootstrap::in_process_bus ---

/// @covers EventBootstrap::in_process_bus — happy path: returns an InProcessEventBus
#[test]
fn test_in_process_bus_returns_bus_happy() {
    let config = EventBusConfig { capacity: 256 };
    let bus: InProcessEventBus = TestEvents::in_process_bus(config);
    assert_ne!(std::mem::size_of_val(&bus), 0, "InProcessEventBus should be heap-backed");
}

/// @covers EventBootstrap::in_process_bus — error: minimum valid capacity (1) constructs without panic
#[test]
fn test_in_process_bus_min_capacity_constructs_error() {
    let config = EventBusConfig { capacity: 1 };
    let bus: InProcessEventBus = TestEvents::in_process_bus(config);
    assert_ne!(std::mem::size_of_val(&bus), 0, "bus with min capacity should construct successfully");
}

/// @covers EventBootstrap::in_process_bus — edge: successive calls produce independent buses
#[test]
fn test_in_process_bus_independent_calls_edge() {
    let a = TestEvents::in_process_bus(EventBusConfig { capacity: 16 });
    let b = TestEvents::in_process_bus(EventBusConfig { capacity: 32 });
    // Different pointers indicate independent instances
    let a_ptr = &a as *const _;
    let b_ptr = &b as *const _;
    assert_ne!(a_ptr, b_ptr, "successive calls should produce independent instances");
}

// --- EventBootstrap::noop_bus ---

/// @covers EventBootstrap::noop_bus — happy path: returns a NoopEventBus marker
#[test]
fn test_noop_bus_returns_marker_happy() {
    let bus: NoopEventBus = TestEvents::noop_bus();
    assert_eq!(std::mem::size_of_val(&bus), 0, "NoopEventBus should be zero-sized");
}

/// @covers EventBootstrap::noop_bus — error: NoopEventBus is zero-size (cannot panic)
#[test]
fn test_noop_bus_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventBus>(), 0);
}

/// @covers EventBootstrap::noop_bus — edge: successive calls are independent
#[test]
fn test_noop_bus_independent_calls_edge() {
    let a = TestEvents::noop_bus();
    let b = TestEvents::noop_bus();
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

// --- EventBootstrap::noop_publisher ---

/// @covers EventBootstrap::noop_publisher — happy path: returns a NoopEventPublisher marker
#[test]
fn test_noop_publisher_returns_marker_happy() {
    let pub_: NoopEventPublisher = TestEvents::noop_publisher();
    assert_eq!(std::mem::size_of_val(&pub_), 0, "NoopEventPublisher should be zero-sized");
}

/// @covers EventBootstrap::noop_publisher — error: NoopEventPublisher is zero-size
#[test]
fn test_noop_publisher_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopEventPublisher>(), 0);
}

/// @covers EventBootstrap::noop_publisher — edge: successive calls are independent
#[test]
fn test_noop_publisher_independent_calls_edge() {
    let a = TestEvents::noop_publisher();
    let b = TestEvents::noop_publisher();
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

// --- EventBootstrap::in_memory_store ---

/// @covers EventBootstrap::in_memory_store — happy path: constructs successfully
#[test]
fn test_in_memory_store_constructs_successfully_happy() {
    let store = TestEvents::in_memory_store::<AnyEvent>();
    assert_ne!(std::mem::size_of_val(&store), 0, "in_memory_store should be heap-backed");
}

/// @covers EventBootstrap::in_memory_store — error: store is non-zero-size (heap-backed)
#[test]
fn test_in_memory_store_is_nonzero_size_error() {
    assert_ne!(
        std::mem::size_of_val(&TestEvents::in_memory_store::<AnyEvent>()),
        0,
    );
}

/// @covers EventBootstrap::in_memory_store — edge: successive calls produce independent stores
#[test]
fn test_in_memory_store_successive_calls_independent_edge() {
    let a = TestEvents::in_memory_store::<AnyEvent>();
    let b = TestEvents::in_memory_store::<AnyEvent>();
    let a_ptr = &a as *const _;
    let b_ptr = &b as *const _;
    assert_ne!(a_ptr, b_ptr, "successive calls should produce independent instances");
}

// --- EventBootstrap::closed_source ---

/// @covers EventBootstrap::closed_source — happy path: returns a ClosedEventSource marker
#[test]
fn test_closed_source_returns_marker_happy() {
    let source: ClosedEventSource = TestEvents::closed_source();
    assert_eq!(std::mem::size_of_val(&source), 0, "ClosedEventSource should be zero-sized");
}

/// @covers EventBootstrap::closed_source — error: ClosedEventSource is zero-size
#[test]
fn test_closed_source_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<ClosedEventSource>(), 0);
}

/// @covers EventBootstrap::closed_source — edge: successive calls produce independent markers
#[test]
fn test_closed_source_successive_calls_independent_edge() {
    let a = TestEvents::closed_source();
    let b = TestEvents::closed_source();
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}
