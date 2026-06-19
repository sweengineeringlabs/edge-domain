//! Tests for `CompletionInput`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    CompletionInput, CompletionMessage, ExecutionConfig, ExecutionMode, MessageRole, ToolDefinition,
};
use serde_json::json;

fn default_config() -> ExecutionConfig {
    ExecutionConfig::new(1024, 30_000, false, false, ExecutionMode::Async)
}

/// @covers: CompletionInput::simple — single-turn with no tools
#[test]
fn test_completion_input_simple_happy() {
    let input = CompletionInput::simple("What is 2+2?", default_config());
    assert_eq!(input.messages.len(), 1);
    assert_eq!(input.messages[0].role, MessageRole::User);
    assert_eq!(input.messages[0].content, "What is 2+2?");
    assert!(input.tools.is_empty());
    assert!(input.system.is_none());
}

/// @covers: CompletionInput::new — multi-turn with system prompt and tools
#[test]
fn test_completion_input_new_happy() {
    let messages = vec![
        CompletionMessage::user("hello"),
        CompletionMessage::assistant("hi"),
        CompletionMessage::user("search for cats"),
    ];
    let tools = vec![ToolDefinition::new("search", "Search", json!({}))];
    let input = CompletionInput::new(
        messages,
        tools,
        Some("You are helpful.".to_string()),
        default_config(),
    );
    assert_eq!(input.messages.len(), 3);
    assert_eq!(input.tools.len(), 1);
    assert_eq!(input.system.as_deref(), Some("You are helpful."));
}

/// @covers: CompletionInput — serializes and deserializes correctly
#[test]
fn test_completion_input_serde_roundtrip_edge() {
    let input = CompletionInput::simple("ping", default_config());
    let json = serde_json::to_string(&input).expect("serialize");
    let back: CompletionInput = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.messages[0].content, "ping");
}

/// @covers: CompletionInput — empty tools list is valid
#[test]
fn test_completion_input_empty_tools_edge() {
    let input = CompletionInput::new(
        vec![CompletionMessage::user("hi")],
        vec![],
        None,
        default_config(),
    );
    assert!(input.tools.is_empty());
}
