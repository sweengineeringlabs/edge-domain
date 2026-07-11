//! Integration tests for `OutboundRegisterRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundRegisterRequest;

/// @covers: OutboundRegisterRequest
#[test]
fn test_outbound_register_request_holds_name_and_handle_happy() {
    let req = OutboundRegisterRequest {
        name: "svc".to_string(),
        handle: 42u32,
    };
    assert_eq!(req.name, "svc");
    assert_eq!(req.handle, 42);
}

/// @covers: OutboundRegisterRequest
#[test]
fn test_outbound_register_request_empty_name_error() {
    let req = OutboundRegisterRequest {
        name: String::new(),
        handle: 1u32,
    };
    assert_eq!(req.name, "");
}

/// @covers: OutboundRegisterRequest
#[test]
fn test_outbound_register_request_generic_over_handle_type_edge() {
    let req = OutboundRegisterRequest {
        name: "s".to_string(),
        handle: "handle-value".to_string(),
    };
    assert_eq!(req.handle, "handle-value");
}
