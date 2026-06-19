//! Tests for `CompletionMessage`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{CompletionMessage, MessageRole};

/// @covers: CompletionMessage::user — constructs user-role message
#[test]
fn test_completion_message_user_happy() {
    let m = CompletionMessage::user("hello");
    assert_eq!(m.role, MessageRole::User);
    assert_eq!(m.content, "hello");
}

/// @covers: CompletionMessage::assistant — constructs assistant-role message
#[test]
fn test_completion_message_assistant_happy() {
    let m = CompletionMessage::assistant("world");
    assert_eq!(m.role, MessageRole::Assistant);
    assert_eq!(m.content, "world");
}

/// @covers: CompletionMessage::tool — constructs tool-role message
#[test]
fn test_completion_message_tool_happy() {
    let m = CompletionMessage::tool("42");
    assert_eq!(m.role, MessageRole::Tool);
    assert_eq!(m.content, "42");
}

/// @covers: CompletionMessage — serializes and deserializes correctly
#[test]
fn test_completion_message_serde_roundtrip_edge() {
    let m = CompletionMessage::user("ping");
    let json = serde_json::to_string(&m).expect("serialize");
    let back: CompletionMessage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.role, MessageRole::User);
    assert_eq!(back.content, "ping");
}
