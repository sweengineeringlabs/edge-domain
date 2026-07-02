//! SAF service tests — model-ops factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::MODEL_OPS_SVC_FACTORY;

/// @covers: MODEL_OPS_SVC_FACTORY — equals the canonical string
#[test]
fn test_model_ops_svc_factory_equals_canonical_happy() {
    assert_eq!(MODEL_OPS_SVC_FACTORY, "model_ops_svc_factory");
}

/// @covers: MODEL_OPS_SVC_FACTORY — is non-empty
#[test]
fn test_model_ops_svc_factory_is_non_empty_error() {
    assert!(!MODEL_OPS_SVC_FACTORY.is_empty());
}

/// @covers: MODEL_OPS_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_model_ops_svc_factory_is_valid_identifier_edge() {
    assert!(MODEL_OPS_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
