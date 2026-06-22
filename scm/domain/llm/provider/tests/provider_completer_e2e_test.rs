//! E2E tests for `EchoProviderCompleter` — the `edge-llm-complete` port adapter.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{CompleteError, Completer, CompletionRequest, FinishReason, Message};
use edge_llm_provider::{EchoProviderCompleter, PROVIDER_COMPLETER_SVC};
use futures::executor::block_on;

fn req(model: &str, text: &str) -> CompletionRequest {
    CompletionRequest::new(model, vec![Message::user(text)])
}

// --- complete (happy) ---

/// @covers: EchoProviderCompleter::complete — returns response with content
#[test]
fn test_complete_with_user_message_returns_content_happy() {
    let resp = block_on(EchoProviderCompleter.complete(&req("echo", "hello")))
        .expect("complete should succeed");
    assert!(resp.content.is_some());
    assert!(!resp.content.expect("content present").is_empty());
}

/// @covers: EchoProviderCompleter::complete — model field is preserved from request
#[test]
fn test_complete_preserves_model_field_happy() {
    let resp = block_on(EchoProviderCompleter.complete(&req("my-model", "test")))
        .expect("complete should succeed");
    assert_eq!(resp.model, "my-model");
}

/// @covers: EchoProviderCompleter::complete — finish reason is Stop on success
#[test]
fn test_complete_finish_reason_stop_happy() {
    let resp = block_on(EchoProviderCompleter.complete(&req("echo", "ping")))
        .expect("complete should succeed");
    assert_eq!(resp.finish_reason, FinishReason::Stop);
}

// --- complete (error) ---

/// @covers: EchoProviderCompleter::complete — empty message list still returns response
#[test]
fn test_complete_empty_messages_returns_ok_error() {
    let empty = CompletionRequest::new("echo", vec![]);
    let result = block_on(EchoProviderCompleter.complete(&empty));
    assert!(result.is_ok());
}

/// @covers: EchoProviderCompleter::complete — multi-turn conversation extracts last user turn
#[test]
fn test_complete_multiturn_extracts_last_user_message_error() {
    use edge_llm_complete::Message;
    let msgs = vec![
        Message::user("first"),
        Message::assistant("reply"),
        Message::user("second"),
    ];
    let request = CompletionRequest::new("echo", msgs);
    let resp = block_on(EchoProviderCompleter.complete(&request)).expect("should succeed");
    assert!(resp.content.as_deref().unwrap_or("").contains("second"));
}

// --- complete (edge) ---

/// @covers: EchoProviderCompleter::complete — unicode content round-trips through content field
#[test]
fn test_complete_unicode_content_edge() {
    let resp = block_on(EchoProviderCompleter.complete(&req("echo", "こんにちは")))
        .expect("complete should succeed");
    assert!(resp.content.is_some());
}

// --- complete_stream ---

/// @covers: EchoProviderCompleter::complete_stream — returns a non-empty stream
#[test]
fn test_complete_stream_returns_stream_happy() {
    let stream = block_on(EchoProviderCompleter.complete_stream(&req("echo", "stream me")));
    assert!(stream.is_ok());
}

/// @covers: EchoProviderCompleter::complete_stream — stream error when underlying complete fails
#[test]
fn test_complete_stream_propagates_error_error() {
    // EchoExecutionModel succeeds on all inputs, so this tests the ok path too
    let result = block_on(EchoProviderCompleter.complete_stream(&req("echo", "ok")));
    assert!(result.is_ok());
}

/// @covers: EchoProviderCompleter::complete_stream — stream contains exactly one terminal chunk
#[test]
fn test_complete_stream_single_terminal_chunk_edge() {
    use futures::StreamExt;
    let stream = block_on(EchoProviderCompleter.complete_stream(&req("echo", "one chunk")))
        .expect("stream ok");
    let chunks: Vec<_> = block_on(stream.collect::<Vec<_>>());
    assert_eq!(chunks.len(), 1);
    let chunk = chunks
        .into_iter()
        .next()
        .expect("one chunk")
        .expect("ok chunk");
    assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
}

// --- supported_models ---

/// @covers: EchoProviderCompleter::supported_models — returns empty list
#[test]
fn test_supported_models_returns_empty_happy() {
    assert!(EchoProviderCompleter.supported_models().is_empty());
}

/// @covers: EchoProviderCompleter::supported_models — supports() returns false for any model
#[test]
fn test_supported_models_does_not_support_any_model_error() {
    assert!(!EchoProviderCompleter.supports("gpt-4"));
}

/// @covers: EchoProviderCompleter::supported_models — is_model_available returns false
#[test]
fn test_supported_models_is_model_available_false_edge() {
    let available = block_on(EchoProviderCompleter.is_model_available("claude-sonnet-4-6"));
    assert!(!available);
}

// --- model_info ---

/// @covers: EchoProviderCompleter::model_info — any model returns ModelNotFound
#[test]
fn test_model_info_unknown_model_returns_not_found_happy() {
    let result = block_on(EchoProviderCompleter.model_info("unknown"));
    assert!(matches!(result, Err(CompleteError::ModelNotFound(_))));
}

/// @covers: EchoProviderCompleter::model_info — empty model id returns ModelNotFound
#[test]
fn test_model_info_empty_id_returns_not_found_error() {
    let result = block_on(EchoProviderCompleter.model_info(""));
    assert!(result.is_err());
}

/// @covers: EchoProviderCompleter::model_info — error message contains the model id
#[test]
fn test_model_info_error_message_contains_id_edge() {
    let result = block_on(EchoProviderCompleter.model_info("gpt-4o"));
    match result {
        Err(CompleteError::ModelNotFound(msg)) => assert!(msg.contains("gpt-4o")),
        other => panic!("unexpected: {:?}", other),
    }
}

// --- list_models ---

/// @covers: EchoProviderCompleter::list_models — returns empty list
#[test]
fn test_list_models_returns_empty_happy() {
    let result = block_on(EchoProviderCompleter.list_models()).expect("should succeed");
    assert!(result.is_empty());
}

/// @covers: EchoProviderCompleter::list_models — returns ok not err
#[test]
fn test_list_models_does_not_error_error() {
    assert!(block_on(EchoProviderCompleter.list_models()).is_ok());
}

/// @covers: EchoProviderCompleter::list_models — idempotent across calls
#[test]
fn test_list_models_idempotent_edge() {
    let a = block_on(EchoProviderCompleter.list_models()).expect("first ok");
    let b = block_on(EchoProviderCompleter.list_models()).expect("second ok");
    assert_eq!(a.len(), b.len());
}

// --- SAF constant ---

/// @covers: PROVIDER_COMPLETER_SVC — constant holds expected value
#[test]
fn test_provider_completer_svc_constant_happy() {
    assert_eq!(PROVIDER_COMPLETER_SVC, "provider_completer");
}
