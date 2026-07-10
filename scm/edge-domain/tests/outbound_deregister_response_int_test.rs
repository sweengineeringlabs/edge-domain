//! Integration tests for `OutboundDeregisterResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundDeregisterResponse;

/// @covers: OutboundDeregisterResponse
#[test]
fn test_outbound_deregister_response_removed_true_happy() {
    let resp = OutboundDeregisterResponse { removed: true };
    assert!(resp.removed);
}

/// @covers: OutboundDeregisterResponse
#[test]
fn test_outbound_deregister_response_removed_false_error() {
    let resp = OutboundDeregisterResponse { removed: false };
    assert!(!resp.removed);
}

/// @covers: OutboundDeregisterResponse
#[test]
fn test_outbound_deregister_response_is_copy_edge() {
    let a = OutboundDeregisterResponse { removed: true };
    let b = a;
    assert_eq!(a, b);
}
