//! SAF service tests — schema-validator factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::SCHEMA_VALIDATOR_SVC_FACTORY;

/// @covers: SCHEMA_VALIDATOR_SVC_FACTORY — equals the canonical string
#[test]
fn test_schema_validator_svc_factory_equals_canonical_happy() {
    assert_eq!(SCHEMA_VALIDATOR_SVC_FACTORY, "schema_validator_svc_factory");
}

/// @covers: SCHEMA_VALIDATOR_SVC_FACTORY — is non-empty
#[test]
fn test_schema_validator_svc_factory_is_non_empty_error() {
    assert!(!SCHEMA_VALIDATOR_SVC_FACTORY.is_empty());
}

/// @covers: SCHEMA_VALIDATOR_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_schema_validator_svc_factory_is_valid_identifier_edge() {
    assert!(SCHEMA_VALIDATOR_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
