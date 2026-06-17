#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `MessageBuilder`.

use edge_llm_agent::{CacheControl, MessageBuilder, MessageContent, Role, ToolCall};

#[test]
fn test_message_builder_defaults_to_empty_user_message() {
    let msg = MessageBuilder::new().build();
    assert_eq!(msg.role, Role::User);
    assert_eq!(msg.content, MessageContent::text(""));
}

#[test]
fn test_message_builder_sets_role_and_content() {
    let msg = MessageBuilder::new()
        .role(Role::System)
        .content("be helpful")
        .build();
    assert_eq!(msg.role, Role::System);
    assert_eq!(msg.content, MessageContent::text("be helpful"));
}

#[test]
fn test_message_builder_accumulates_tool_calls_and_cache_control() {
    let msg = MessageBuilder::new()
        .name("alice")
        .tool_call_id("call-1")
        .tool_call(ToolCall {
            id: "call-1".to_string(),
            name: "search".to_string(),
            arguments: "{}".to_string(),
        })
        .cache_control(CacheControl::ephemeral())
        .build();
    assert_eq!(msg.name.as_deref(), Some("alice"));
    assert_eq!(msg.tool_calls.len(), 1);
    assert!(msg.cache_control.is_some());
}
