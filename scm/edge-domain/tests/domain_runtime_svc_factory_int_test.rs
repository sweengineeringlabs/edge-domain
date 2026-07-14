//! Integration test for the `DomainRuntime` SAF module anchor.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::DOMAIN_RUNTIME_SVC;

/// @covers: DOMAIN_RUNTIME_SVC
#[test]
fn test_domain_runtime_svc_anchor_is_unit_happy() {
    assert_eq!(DOMAIN_RUNTIME_SVC, ());
}

/// @covers: DOMAIN_RUNTIME_SVC
#[test]
fn test_domain_runtime_svc_anchor_is_accessible_from_crate_root_error() {
    assert_eq!(edge_application::DOMAIN_RUNTIME_SVC, ());
}

/// @covers: DOMAIN_RUNTIME_SVC
#[test]
fn test_domain_runtime_svc_anchor_debug_format_edge() {
    assert_eq!(format!("{DOMAIN_RUNTIME_SVC:?}"), "()");
}
