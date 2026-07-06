//! SAF service tests — provider-bootstrap factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::PROVIDER_BOOTSTRAP_SVC_FACTORY;

/// @covers: PROVIDER_BOOTSTRAP_SVC_FACTORY — equals the canonical string
#[test]
fn test_provider_bootstrap_svc_factory_equals_canonical_happy() {
    assert_eq!(PROVIDER_BOOTSTRAP_SVC_FACTORY, "provider_bootstrap_factory");
}

/// @covers: PROVIDER_BOOTSTRAP_SVC_FACTORY — is non-empty
#[test]
fn test_provider_bootstrap_svc_factory_is_non_empty_error() {
    assert!(!PROVIDER_BOOTSTRAP_SVC_FACTORY.is_empty());
}

/// @covers: PROVIDER_BOOTSTRAP_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_provider_bootstrap_svc_factory_is_valid_identifier_edge() {
    assert!(PROVIDER_BOOTSTRAP_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
