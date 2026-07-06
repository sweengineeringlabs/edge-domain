//! SAF service tests — tool-call-loop factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::TOOL_CALL_LOOP_SVC_FACTORY;

/// @covers: TOOL_CALL_LOOP_SVC_FACTORY — equals the canonical string
#[test]
fn test_tool_call_loop_svc_factory_equals_canonical_happy() {
    assert_eq!(TOOL_CALL_LOOP_SVC_FACTORY, "tool_call_loop_svc_factory");
}

/// @covers: TOOL_CALL_LOOP_SVC_FACTORY — is non-empty
#[test]
fn test_tool_call_loop_svc_factory_is_non_empty_error() {
    assert!(!TOOL_CALL_LOOP_SVC_FACTORY.is_empty());
}

/// @covers: TOOL_CALL_LOOP_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_tool_call_loop_svc_factory_is_valid_identifier_edge() {
    assert!(TOOL_CALL_LOOP_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
