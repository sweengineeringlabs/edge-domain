//! SAF service tests — conversation-loop factory marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::CONVERSATION_LOOP_SVC_FACTORY;

/// @covers: CONVERSATION_LOOP_SVC_FACTORY — equals the canonical string
#[test]
fn test_conversation_loop_svc_factory_equals_canonical_happy() {
    assert_eq!(
        CONVERSATION_LOOP_SVC_FACTORY,
        "conversation_loop_svc_factory"
    );
}

/// @covers: CONVERSATION_LOOP_SVC_FACTORY — is non-empty
#[test]
fn test_conversation_loop_svc_factory_is_non_empty_error() {
    assert!(!CONVERSATION_LOOP_SVC_FACTORY.is_empty());
}

/// @covers: CONVERSATION_LOOP_SVC_FACTORY — contains only lowercase ASCII and underscores
#[test]
fn test_conversation_loop_svc_factory_is_valid_identifier_edge() {
    assert!(CONVERSATION_LOOP_SVC_FACTORY
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
