//! SAF service tests — provider contract identifier.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::PROVIDER_SVC;

/// @covers: PROVIDER_SVC — equals the canonical string
#[test]
fn test_provider_svc_equals_canonical_happy() {
    assert_eq!(PROVIDER_SVC, "provider");
}

/// @covers: PROVIDER_SVC — is non-empty
#[test]
fn test_provider_svc_is_non_empty_error() {
    assert!(!PROVIDER_SVC.is_empty());
}

/// @covers: PROVIDER_SVC — contains only lowercase ASCII and underscores
#[test]
fn test_provider_svc_is_valid_identifier_edge() {
    assert!(PROVIDER_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
