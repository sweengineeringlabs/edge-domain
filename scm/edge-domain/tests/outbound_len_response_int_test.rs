//! Integration tests for `OutboundLenResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::OutboundLenResponse;

/// @covers: OutboundLenResponse
#[test]
fn test_outbound_len_response_holds_count_happy() {
    let resp = OutboundLenResponse { count: 3 };
    assert_eq!(resp.count, 3);
}

/// @covers: OutboundLenResponse
#[test]
fn test_outbound_len_response_zero_count_error() {
    let resp = OutboundLenResponse { count: 0 };
    assert_eq!(resp.count, 0);
}

/// @covers: OutboundLenResponse
#[test]
fn test_outbound_len_response_is_copy_edge() {
    let a = OutboundLenResponse { count: 5 };
    let b = a;
    assert_eq!(a, b);
}
