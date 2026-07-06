//! SAF service tests — execution-model factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::EXECUTION_MODEL_SVC_FACTORY;

/// @covers: EXECUTION_MODEL_SVC_FACTORY — equals the canonical string
#[test]
fn test_execution_model_svc_factory_equals_canonical_happy() {
    assert_eq!(EXECUTION_MODEL_SVC_FACTORY, "execution_model_factory");
}

/// @covers: EXECUTION_MODEL_SVC_FACTORY — is non-empty
#[test]
fn test_execution_model_svc_factory_is_non_empty_error() {
    assert!(!EXECUTION_MODEL_SVC_FACTORY.is_empty());
}

/// @covers: EXECUTION_MODEL_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_execution_model_svc_factory_is_valid_identifier_edge() {
    assert!(EXECUTION_MODEL_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
