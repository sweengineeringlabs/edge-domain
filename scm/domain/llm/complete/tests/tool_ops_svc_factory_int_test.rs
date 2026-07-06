//! SAF service tests — tool-ops factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::TOOL_OPS_SVC_FACTORY;

/// @covers: TOOL_OPS_SVC_FACTORY — equals the canonical string
#[test]
fn test_tool_ops_svc_factory_equals_canonical_happy() {
    assert_eq!(TOOL_OPS_SVC_FACTORY, "tool_ops_svc_factory");
}

/// @covers: TOOL_OPS_SVC_FACTORY — is non-empty
#[test]
fn test_tool_ops_svc_factory_is_non_empty_error() {
    assert!(!TOOL_OPS_SVC_FACTORY.is_empty());
}

/// @covers: TOOL_OPS_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_tool_ops_svc_factory_is_valid_identifier_edge() {
    assert!(TOOL_OPS_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
