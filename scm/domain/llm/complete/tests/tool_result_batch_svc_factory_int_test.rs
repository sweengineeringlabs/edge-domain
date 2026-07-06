//! SAF service tests — tool-result-batch factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::TOOL_RESULT_BATCH_SVC_FACTORY;

/// @covers: TOOL_RESULT_BATCH_SVC_FACTORY — equals the canonical string
#[test]
fn test_tool_result_batch_svc_factory_equals_canonical_happy() {
    assert_eq!(
        TOOL_RESULT_BATCH_SVC_FACTORY,
        "tool_result_batch_svc_factory"
    );
}

/// @covers: TOOL_RESULT_BATCH_SVC_FACTORY — is non-empty
#[test]
fn test_tool_result_batch_svc_factory_is_non_empty_error() {
    assert!(!TOOL_RESULT_BATCH_SVC_FACTORY.is_empty());
}

/// @covers: TOOL_RESULT_BATCH_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_tool_result_batch_svc_factory_is_valid_identifier_edge() {
    assert!(TOOL_RESULT_BATCH_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
