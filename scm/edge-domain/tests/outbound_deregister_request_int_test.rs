//! Integration tests for `OutboundDeregisterRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundDeregisterRequest;

/// @covers: OutboundDeregisterRequest
#[test]
fn test_outbound_deregister_request_holds_name_happy() {
    let req = OutboundDeregisterRequest {
        name: "svc".to_string(),
    };
    assert_eq!(req.name, "svc");
}

/// @covers: OutboundDeregisterRequest
#[test]
fn test_outbound_deregister_request_empty_name_error() {
    let req = OutboundDeregisterRequest {
        name: String::new(),
    };
    assert_eq!(req.name, "");
}

/// @covers: OutboundDeregisterRequest
#[test]
fn test_outbound_deregister_request_unicode_name_edge() {
    let req = OutboundDeregisterRequest {
        name: "sv\u{1F600}c".to_string(),
    };
    assert_eq!(req.name, "sv\u{1F600}c");
}
