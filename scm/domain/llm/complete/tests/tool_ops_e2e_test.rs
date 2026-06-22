//! Scenario coverage for the `ToolOps` trait.

use edge_llm_complete::{
    CompleteError, ToolCall, ToolCallDelta, ToolChoice, ToolDefinition, ToolOps,
};
use serde_json::json;

struct EchoToolOps;

impl ToolOps for EchoToolOps {
    fn execute(&self, call: &ToolCall) -> Result<String, CompleteError> {
        if call.name.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "tool name required".to_string(),
            ));
        }
        Ok(format!("result:{}", call.name))
    }

    fn available_tools(&self) -> Vec<ToolDefinition> {
        vec![ToolDefinition::new("search", "Search", json!({}))]
    }

    fn tool_choice(&self) -> ToolChoice {
        ToolChoice::Auto
    }

    fn merge_delta(&self, existing: &mut ToolCallDelta, incoming: ToolCallDelta) {
        if let Some(args) = incoming.arguments {
            let base = existing.arguments.get_or_insert_with(String::new);
            base.push_str(&args);
        }
    }
}

// ── execute ───────────────────────────────────────────────────────────────────

#[test]
fn test_execute_valid_call_returns_result_happy() {
    let call = ToolCall::new("id-1", "search", r#"{"q":"rust"}"#);
    let result = EchoToolOps.execute(&call).unwrap();
    assert_eq!(result, "result:search");
}

#[test]
fn test_execute_empty_name_returns_error_error() {
    let call = ToolCall::new("id-1", "", "{}");
    let err = EchoToolOps.execute(&call).unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}

#[test]
fn test_execute_complex_arguments_is_valid_edge() {
    let call = ToolCall::new("x", "search", r#"{"q":"hello world","limit":10}"#);
    let result = EchoToolOps.execute(&call).unwrap();
    assert!(result.contains("search"));
}

// ── available_tools ───────────────────────────────────────────────────────────

#[test]
fn test_available_tools_returns_non_empty_list_happy() {
    assert!(!EchoToolOps.available_tools().is_empty());
}

#[test]
fn test_available_tools_entries_have_names_error() {
    for t in EchoToolOps.available_tools() {
        assert!(!t.name.is_empty());
    }
}

#[test]
fn test_available_tools_returns_owned_vec_edge() {
    let tools = EchoToolOps.available_tools();
    assert_eq!(tools.len(), 1);
}

// ── tool_choice ───────────────────────────────────────────────────────────────

#[test]
fn test_tool_choice_returns_auto_happy() {
    assert_eq!(EchoToolOps.tool_choice(), ToolChoice::Auto);
}

#[test]
fn test_tool_choice_is_not_none_error() {
    assert_ne!(EchoToolOps.tool_choice(), ToolChoice::None);
}

#[test]
fn test_tool_choice_is_not_required_edge() {
    assert_ne!(EchoToolOps.tool_choice(), ToolChoice::Required);
}

// ── merge_delta ───────────────────────────────────────────────────────────────

#[test]
fn test_merge_delta_appends_arguments_happy() {
    let mut existing = ToolCallDelta::new(0).with_arguments("{");
    let incoming = ToolCallDelta::new(0).with_arguments(r#""key": "val"}"#);
    EchoToolOps.merge_delta(&mut existing, incoming);
    assert_eq!(existing.arguments, Some(r#"{"key": "val"}"#.to_string()));
}

#[test]
fn test_merge_delta_no_arguments_is_noop_error() {
    let mut existing = ToolCallDelta::new(0);
    let incoming = ToolCallDelta::new(0);
    EchoToolOps.merge_delta(&mut existing, incoming);
    assert!(existing.arguments.is_none());
}

#[test]
fn test_merge_delta_accumulates_multiple_fragments_edge() {
    let mut existing = ToolCallDelta::new(0).with_arguments("a");
    EchoToolOps.merge_delta(&mut existing, ToolCallDelta::new(0).with_arguments("b"));
    EchoToolOps.merge_delta(&mut existing, ToolCallDelta::new(0).with_arguments("c"));
    assert_eq!(existing.arguments, Some("abc".to_string()));
}
