//! Integration tests for `OutboundGetResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundGetResponse;

/// @covers: OutboundGetResponse
#[test]
fn test_outbound_get_response_some_handle_happy() {
    let resp = OutboundGetResponse {
        handle: Some(42u32),
    };
    assert_eq!(resp.handle, Some(42));
}

/// @covers: OutboundGetResponse
#[test]
fn test_outbound_get_response_none_handle_error() {
    let resp: OutboundGetResponse<u32> = OutboundGetResponse { handle: None };
    assert_eq!(resp.handle, None);
}

/// @covers: OutboundGetResponse
#[test]
fn test_outbound_get_response_generic_over_handle_type_edge() {
    let resp = OutboundGetResponse {
        handle: Some("v".to_string()),
    };
    assert_eq!(resp.handle.as_deref(), Some("v"));
}
