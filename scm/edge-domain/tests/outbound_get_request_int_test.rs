//! Integration tests for `OutboundGetRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundGetRequest;

/// @covers: OutboundGetRequest
#[test]
fn test_outbound_get_request_holds_name_happy() {
    let req = OutboundGetRequest {
        name: "svc".to_string(),
    };
    assert_eq!(req.name, "svc");
}

/// @covers: OutboundGetRequest
#[test]
fn test_outbound_get_request_empty_name_error() {
    let req = OutboundGetRequest {
        name: String::new(),
    };
    assert_eq!(req.name, "");
}

/// @covers: OutboundGetRequest
#[test]
fn test_outbound_get_request_unicode_name_edge() {
    let req = OutboundGetRequest {
        name: "sv\u{1F600}c".to_string(),
    };
    assert_eq!(req.name, "sv\u{1F600}c");
}
