//! Scenario coverage for the `ToolOps` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    AvailableToolsRequest, AvailableToolsResponse, CompleteError, DeltaMergeRequest, ToolCall,
    ToolCallDelta, ToolChoice, ToolChoicePreferenceRequest, ToolChoicePreferenceResponse,
    ToolDefinition, ToolExecutionRequest, ToolExecutionResponse, ToolOps,
};
use serde_json::json;

struct EchoToolOps;

impl ToolOps for EchoToolOps {
    fn execute(
        &self,
        req: ToolExecutionRequest<'_>,
    ) -> Result<ToolExecutionResponse, CompleteError> {
        let call = req.call;
        if call.name.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "tool name required".to_string(),
            ));
        }
        Ok(ToolExecutionResponse {
            output: format!("result:{}", call.name),
        })
    }

    fn available_tools(
        &self,
        _req: AvailableToolsRequest,
    ) -> Result<AvailableToolsResponse, CompleteError> {
        Ok(AvailableToolsResponse {
            tools: vec![ToolDefinition::new("search", "Search", json!({}))],
        })
    }

    fn tool_choice(
        &self,
        _req: ToolChoicePreferenceRequest,
    ) -> Result<ToolChoicePreferenceResponse, CompleteError> {
        Ok(ToolChoicePreferenceResponse {
            choice: ToolChoice::Auto,
        })
    }

    fn merge_delta(&self, req: DeltaMergeRequest<'_>) -> Result<(), CompleteError> {
        let DeltaMergeRequest { existing, incoming } = req;
        if let Some(args) = incoming.arguments {
            let base = existing.arguments.get_or_insert_with(String::new);
            base.push_str(&args);
        }
        Ok(())
    }
}

// ── execute ───────────────────────────────────────────────────────────────────

#[test]
fn test_execute_valid_call_returns_result_happy() {
    let call = ToolCall::new("id-1", "search", r#"{"q":"rust"}"#);
    let result = EchoToolOps
        .execute(ToolExecutionRequest { call: &call })
        .unwrap();
    assert_eq!(result.output, "result:search");
}

#[test]
fn test_execute_empty_name_returns_error_error() {
    let call = ToolCall::new("id-1", "", "{}");
    let err = EchoToolOps
        .execute(ToolExecutionRequest { call: &call })
        .unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}

#[test]
fn test_execute_complex_arguments_is_valid_edge() {
    let call = ToolCall::new("x", "search", r#"{"q":"hello world","limit":10}"#);
    let result = EchoToolOps
        .execute(ToolExecutionRequest { call: &call })
        .unwrap();
    assert!(result.output.contains("search"));
}

// ── available_tools ───────────────────────────────────────────────────────────

#[test]
fn test_available_tools_returns_non_empty_list_happy() {
    let tools = EchoToolOps
        .available_tools(AvailableToolsRequest)
        .unwrap()
        .tools;
    assert!(!tools.is_empty());
}

#[test]
fn test_available_tools_entries_have_names_error() {
    let tools = EchoToolOps
        .available_tools(AvailableToolsRequest)
        .unwrap()
        .tools;
    for t in tools {
        assert!(!t.name.is_empty());
    }
}

#[test]
fn test_available_tools_returns_owned_vec_edge() {
    let tools = EchoToolOps
        .available_tools(AvailableToolsRequest)
        .unwrap()
        .tools;
    assert_eq!(tools.len(), 1);
}

// ── tool_choice ───────────────────────────────────────────────────────────────

#[test]
fn test_tool_choice_returns_auto_happy() {
    let choice = EchoToolOps
        .tool_choice(ToolChoicePreferenceRequest)
        .unwrap()
        .choice;
    assert_eq!(choice, ToolChoice::Auto);
}

#[test]
fn test_tool_choice_is_not_none_error() {
    let choice = EchoToolOps
        .tool_choice(ToolChoicePreferenceRequest)
        .unwrap()
        .choice;
    assert_ne!(choice, ToolChoice::None);
}

#[test]
fn test_tool_choice_is_not_required_edge() {
    let choice = EchoToolOps
        .tool_choice(ToolChoicePreferenceRequest)
        .unwrap()
        .choice;
    assert_ne!(choice, ToolChoice::Required);
}

// ── merge_delta ───────────────────────────────────────────────────────────────

#[test]
fn test_merge_delta_appends_arguments_happy() {
    let mut existing = ToolCallDelta::new(0).with_arguments("{");
    let incoming = ToolCallDelta::new(0).with_arguments(r#""key": "val"}"#);
    EchoToolOps
        .merge_delta(DeltaMergeRequest {
            existing: &mut existing,
            incoming: Box::new(incoming),
        })
        .unwrap();
    assert_eq!(existing.arguments, Some(r#"{"key": "val"}"#.to_string()));
}

#[test]
fn test_merge_delta_no_arguments_is_noop_error() {
    let mut existing = ToolCallDelta::new(0);
    let incoming = ToolCallDelta::new(0);
    EchoToolOps
        .merge_delta(DeltaMergeRequest {
            existing: &mut existing,
            incoming: Box::new(incoming),
        })
        .unwrap();
    assert!(existing.arguments.is_none());
}

#[test]
fn test_merge_delta_accumulates_multiple_fragments_edge() {
    let mut existing = ToolCallDelta::new(0).with_arguments("a");
    EchoToolOps
        .merge_delta(DeltaMergeRequest {
            existing: &mut existing,
            incoming: Box::new(ToolCallDelta::new(0).with_arguments("b")),
        })
        .unwrap();
    EchoToolOps
        .merge_delta(DeltaMergeRequest {
            existing: &mut existing,
            incoming: Box::new(ToolCallDelta::new(0).with_arguments("c")),
        })
        .unwrap();
    assert_eq!(existing.arguments, Some("abc".to_string()));
}
