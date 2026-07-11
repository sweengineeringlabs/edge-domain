//! Integration tests for `OutboundIsEmptyResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundIsEmptyResponse;

/// @covers: OutboundIsEmptyResponse
#[test]
fn test_outbound_is_empty_response_true_happy() {
    let resp = OutboundIsEmptyResponse { empty: true };
    assert!(resp.empty);
}

/// @covers: OutboundIsEmptyResponse
#[test]
fn test_outbound_is_empty_response_false_error() {
    let resp = OutboundIsEmptyResponse { empty: false };
    assert!(!resp.empty);
}

/// @covers: OutboundIsEmptyResponse
#[test]
fn test_outbound_is_empty_response_is_copy_edge() {
    let a = OutboundIsEmptyResponse { empty: true };
    let b = a;
    assert_eq!(a, b);
}
