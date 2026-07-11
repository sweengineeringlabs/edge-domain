//! Integration tests for `OutboundIsEmptyRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundIsEmptyRequest;

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_default_happy() {
    assert_eq!(OutboundIsEmptyRequest::default(), OutboundIsEmptyRequest);
}

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_debug_format_error() {
    assert_eq!(
        format!("{OutboundIsEmptyRequest:?}"),
        "OutboundIsEmptyRequest"
    );
}

/// @covers: OutboundIsEmptyRequest
#[test]
fn test_outbound_is_empty_request_is_copy_edge() {
    let a = OutboundIsEmptyRequest;
    let b = a;
    assert_eq!(a, b);
}
