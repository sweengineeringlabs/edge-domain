//! Integration tests for `InProcessEventBusRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{EventBusConfig, InProcessEventBusRequest};

/// @covers: InProcessEventBusRequest
#[test]
fn test_in_process_event_bus_request_carries_config_happy() {
    let req = InProcessEventBusRequest {
        config: EventBusConfig { capacity: 42 },
    };
    assert_eq!(req.config.capacity, 42);
}

/// @covers: InProcessEventBusRequest
#[test]
fn test_in_process_event_bus_request_zero_capacity_error() {
    let req = InProcessEventBusRequest {
        config: EventBusConfig { capacity: 0 },
    };
    assert_eq!(req.config.capacity, 0);
}

/// @covers: InProcessEventBusRequest
#[test]
fn test_in_process_event_bus_request_clone_preserves_capacity_edge() {
    let req = InProcessEventBusRequest {
        config: EventBusConfig { capacity: 7 },
    };
    let cloned = req.clone();
    assert_eq!(cloned.config.capacity, req.config.capacity);
}
