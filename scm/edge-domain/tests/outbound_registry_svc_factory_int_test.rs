//! Integration test for the `OutboundRegistry` SAF module anchor.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::OUTBOUND_REGISTRY_SVC;

/// @covers: OUTBOUND_REGISTRY_SVC
#[test]
fn test_outbound_registry_svc_anchor_is_unit_happy() {
    assert_eq!(OUTBOUND_REGISTRY_SVC, ());
}

/// @covers: OUTBOUND_REGISTRY_SVC
#[test]
fn test_outbound_registry_svc_anchor_is_accessible_from_crate_root_error() {
    assert_eq!(edge_domain::OUTBOUND_REGISTRY_SVC, ());
}

/// @covers: OUTBOUND_REGISTRY_SVC
#[test]
fn test_outbound_registry_svc_anchor_debug_format_edge() {
    assert_eq!(format!("{OUTBOUND_REGISTRY_SVC:?}"), "()");
}
