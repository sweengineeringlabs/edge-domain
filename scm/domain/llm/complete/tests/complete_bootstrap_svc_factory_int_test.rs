//! SAF service tests — complete-bootstrap factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::COMPLETE_BOOTSTRAP_SVC_FACTORY;

/// @covers: COMPLETE_BOOTSTRAP_SVC_FACTORY — equals the canonical string
#[test]
fn test_complete_bootstrap_svc_factory_equals_canonical_happy() {
    assert_eq!(
        COMPLETE_BOOTSTRAP_SVC_FACTORY,
        "complete_bootstrap_svc_factory"
    );
}

/// @covers: COMPLETE_BOOTSTRAP_SVC_FACTORY — is non-empty
#[test]
fn test_complete_bootstrap_svc_factory_is_non_empty_error() {
    assert!(!COMPLETE_BOOTSTRAP_SVC_FACTORY.is_empty());
}

/// @covers: COMPLETE_BOOTSTRAP_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_complete_bootstrap_svc_factory_is_valid_identifier_edge() {
    assert!(COMPLETE_BOOTSTRAP_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
