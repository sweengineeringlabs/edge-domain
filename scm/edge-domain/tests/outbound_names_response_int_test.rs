//! Integration tests for `OutboundNamesResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundNamesResponse;

/// @covers: OutboundNamesResponse
#[test]
fn test_outbound_names_response_holds_names_happy() {
    let resp = OutboundNamesResponse {
        names: vec!["a".to_string(), "b".to_string()],
    };
    assert_eq!(resp.names, vec!["a", "b"]);
}

/// @covers: OutboundNamesResponse
#[test]
fn test_outbound_names_response_empty_vec_error() {
    let resp = OutboundNamesResponse { names: vec![] };
    assert!(resp.names.is_empty());
}

/// @covers: OutboundNamesResponse
#[test]
fn test_outbound_names_response_preserves_order_edge() {
    let resp = OutboundNamesResponse {
        names: vec!["z".to_string(), "a".to_string()],
    };
    assert_eq!(resp.names[0], "z");
    assert_eq!(resp.names[1], "a");
}
