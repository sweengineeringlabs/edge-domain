//! Integration tests for `OutboundRegisterResponse`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OutboundRegisterResponse;

/// @covers: OutboundRegisterResponse
#[test]
fn test_outbound_register_response_default_happy() {
    assert_eq!(
        OutboundRegisterResponse::default(),
        OutboundRegisterResponse
    );
}

/// @covers: OutboundRegisterResponse
#[test]
fn test_outbound_register_response_debug_format_error() {
    assert_eq!(
        format!("{OutboundRegisterResponse:?}"),
        "OutboundRegisterResponse"
    );
}

/// @covers: OutboundRegisterResponse
#[test]
fn test_outbound_register_response_is_copy_edge() {
    let a = OutboundRegisterResponse;
    let b = a;
    assert_eq!(a, b);
}
