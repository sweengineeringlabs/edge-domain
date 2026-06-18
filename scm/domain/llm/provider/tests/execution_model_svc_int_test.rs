//! SAF service tests — execution-model contract identifier.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::EXECUTION_MODEL_SVC;

/// @covers: EXECUTION_MODEL_SVC — equals the canonical string
#[test]
fn test_execution_model_svc_equals_canonical_happy() {
    assert_eq!(EXECUTION_MODEL_SVC, "execution_model");
}

/// @covers: EXECUTION_MODEL_SVC — is non-empty
#[test]
fn test_execution_model_svc_is_non_empty_error() {
    assert!(!EXECUTION_MODEL_SVC.is_empty());
}

/// @covers: EXECUTION_MODEL_SVC — contains only lowercase ASCII and underscores
#[test]
fn test_execution_model_svc_is_valid_identifier_edge() {
    assert!(EXECUTION_MODEL_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
