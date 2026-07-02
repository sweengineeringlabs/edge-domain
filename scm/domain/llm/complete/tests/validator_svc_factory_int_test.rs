//! SAF service tests — validator factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::VALIDATOR_SVC_FACTORY;

/// @covers: VALIDATOR_SVC_FACTORY — equals the canonical string
#[test]
fn test_validator_svc_factory_equals_canonical_happy() {
    assert_eq!(VALIDATOR_SVC_FACTORY, "validator_svc_factory");
}

/// @covers: VALIDATOR_SVC_FACTORY — is non-empty
#[test]
fn test_validator_svc_factory_is_non_empty_error() {
    assert!(!VALIDATOR_SVC_FACTORY.is_empty());
}

/// @covers: VALIDATOR_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_validator_svc_factory_is_valid_identifier_edge() {
    assert!(VALIDATOR_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
