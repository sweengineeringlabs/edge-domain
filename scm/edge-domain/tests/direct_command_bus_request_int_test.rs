//! Integration tests for `DirectCommandBusRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::DirectCommandBusRequest;

/// @covers: DirectCommandBusRequest
#[test]
fn test_direct_command_bus_request_default_happy() {
    assert_eq!(DirectCommandBusRequest::default(), DirectCommandBusRequest);
}

/// @covers: DirectCommandBusRequest
#[test]
fn test_direct_command_bus_request_debug_format_error() {
    assert_eq!(
        format!("{DirectCommandBusRequest:?}"),
        "DirectCommandBusRequest"
    );
}

/// @covers: DirectCommandBusRequest
#[test]
fn test_direct_command_bus_request_is_copy_edge() {
    let a = DirectCommandBusRequest;
    let b = a;
    assert_eq!(a, b);
}
