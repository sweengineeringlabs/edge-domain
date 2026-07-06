//! Scenario coverage for the `conversation_loop_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::CONVERSATION_LOOP_SVC;

#[test]
fn test_conversation_loop_svc_constant_is_expected_value_happy() {
    assert_eq!(CONVERSATION_LOOP_SVC, "conversation_loop");
}

#[test]
fn test_conversation_loop_svc_constant_is_nonempty_error() {
    assert!(!CONVERSATION_LOOP_SVC.is_empty());
}

#[test]
fn test_conversation_loop_svc_constant_is_valid_identifier_edge() {
    assert!(CONVERSATION_LOOP_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
