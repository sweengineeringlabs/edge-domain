//! Scenario coverage for the `CompleteOps` trait.

use edge_llm_complete::{
    CompleteError, CompleteOps, CompletionRequest, CompletionResponse, FinishReason, Message,
    TokenUsage,
};

struct StdOps;

impl CompleteOps for StdOps {
    fn check(&self, req: &CompletionRequest) -> Result<(), CompleteError> {
        if req.model.is_empty() {
            Err(CompleteError::InvalidRequest("no model".to_string()))
        } else {
            Ok(())
        }
    }
}

// ── assemble ──────────────────────────────────────────────────────────────────

#[test]
fn test_assemble_produces_request_with_correct_model_happy() {
    let req = StdOps::assemble("gpt-4".to_string(), vec![Message::user("hi")]);
    assert_eq!(req.model, "gpt-4");
}

#[test]
fn test_assemble_empty_messages_is_valid_error() {
    let req = StdOps::assemble("m".to_string(), vec![]);
    assert!(req.messages.is_empty());
}

#[test]
fn test_assemble_preserves_message_count_edge() {
    let msgs = vec![Message::user("a"), Message::user("b"), Message::user("c")];
    let req = StdOps::assemble("m".to_string(), msgs);
    assert_eq!(req.messages.len(), 3);
}

// ── extract_usage ─────────────────────────────────────────────────────────────

#[test]
fn test_extract_usage_returns_usage_ref_happy() {
    let resp = CompletionResponse {
        usage: Box::new(TokenUsage::new(10, 5, 15, 0, 0)),
        ..Default::default()
    };
    let u = StdOps::extract_usage(&resp);
    assert_eq!(u.total_tokens, 15);
}

#[test]
fn test_extract_usage_zero_tokens_error() {
    let resp = CompletionResponse::default();
    let u = StdOps::extract_usage(&resp);
    assert_eq!(u.total_tokens, 0);
}

#[test]
fn test_extract_usage_cache_tokens_accessible_edge() {
    let resp = CompletionResponse {
        usage: Box::new(TokenUsage::new(0, 0, 0, 100, 50)),
        ..Default::default()
    };
    let u = StdOps::extract_usage(&resp);
    assert_eq!(u.cache_read_input_tokens, 100);
}

// ── extract_finish ────────────────────────────────────────────────────────────

#[test]
fn test_extract_finish_returns_stop_by_default_happy() {
    let resp = CompletionResponse::default();
    assert_eq!(*StdOps::extract_finish(&resp), FinishReason::Stop);
}

#[test]
fn test_extract_finish_length_reason_error() {
    let resp = CompletionResponse {
        finish_reason: FinishReason::Length,
        ..Default::default()
    };
    assert_eq!(*StdOps::extract_finish(&resp), FinishReason::Length);
}

#[test]
fn test_extract_finish_tool_calls_reason_edge() {
    let resp = CompletionResponse {
        finish_reason: FinishReason::ToolCalls,
        ..Default::default()
    };
    assert_eq!(*StdOps::extract_finish(&resp), FinishReason::ToolCalls);
}

// ── create_response ───────────────────────────────────────────────────────────

#[test]
fn test_create_response_sets_id_and_model_happy() {
    let resp = StdOps::create_response("r-1".to_string(), "gpt-4".to_string());
    assert_eq!(resp.id, "r-1");
    assert_eq!(resp.model, "gpt-4");
}

#[test]
fn test_create_response_empty_id_is_valid_error() {
    let resp = StdOps::create_response(String::new(), "m".to_string());
    assert!(resp.id.is_empty());
}

#[test]
fn test_create_response_content_is_none_by_default_edge() {
    let resp = StdOps::create_response("x".to_string(), "y".to_string());
    assert!(resp.content.is_none());
}

// ── create_usage ──────────────────────────────────────────────────────────────

#[test]
fn test_create_usage_is_all_zero_happy() {
    let u = StdOps::create_usage();
    assert_eq!(u.total_tokens, 0);
}

#[test]
fn test_create_usage_prompt_tokens_zero_error() {
    let u = StdOps::create_usage();
    assert_eq!(u.prompt_tokens, 0);
}

#[test]
fn test_create_usage_cache_tokens_zero_edge() {
    let u = StdOps::create_usage();
    assert_eq!(u.cache_creation_input_tokens, 0);
}

// ── check ─────────────────────────────────────────────────────────────────────

#[test]
fn test_check_valid_request_returns_ok_happy() {
    let req = CompletionRequest::new("gpt-4", vec![]);
    assert!(StdOps.check(&req).is_ok());
}

#[test]
fn test_check_empty_model_returns_error_error() {
    let req = CompletionRequest::new("", vec![]);
    assert!(StdOps.check(&req).is_err());
}

#[test]
fn test_check_long_model_name_is_valid_edge() {
    let req = CompletionRequest::new("a".repeat(200), vec![]);
    assert!(StdOps.check(&req).is_ok());
}
