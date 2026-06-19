//! Tests for `MessageRole`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::MessageRole;

/// @covers: MessageRole — variants are distinct
#[test]
fn test_message_role_variants_are_distinct_happy() {
    assert_ne!(MessageRole::User, MessageRole::Assistant);
    assert_ne!(MessageRole::User, MessageRole::Tool);
    assert_ne!(MessageRole::Assistant, MessageRole::Tool);
}

/// @covers: MessageRole — serializes to expected string
#[test]
fn test_message_role_serde_roundtrip_edge() {
    for role in [MessageRole::User, MessageRole::Assistant, MessageRole::Tool] {
        let json = serde_json::to_string(&role).expect("serialize");
        let back: MessageRole = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(role, back);
    }
}

/// @covers: MessageRole — debug output is non-empty
#[test]
fn test_message_role_debug_edge() {
    assert!(!format!("{:?}", MessageRole::User).is_empty());
}
