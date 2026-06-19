//! Scenario coverage for the `Completer` trait.

use edge_llm_complete::{
    CompleteError, CompletionRequest, Completer, EchoCompleter, Message, NoopCompleter,
};
use futures::executor::block_on;
use futures::StreamExt;

// ── complete ────────────────────────────────────────────────────────────────

#[test]
fn test_complete_echo_returns_user_content_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("hello")]);
    let resp = block_on(EchoCompleter.complete(&req)).unwrap();
    assert_eq!(resp.content, Some("hello".to_string()));
}

#[test]
fn test_complete_noop_returns_provider_not_found_error() {
    let req = CompletionRequest::new("x", vec![]);
    let err = block_on(NoopCompleter.complete(&req)).unwrap_err();
    assert!(matches!(err, CompleteError::ProviderNotFound(_)));
}

#[test]
fn test_complete_empty_messages_returns_empty_content_edge() {
    let req = CompletionRequest::new("echo", vec![]);
    let resp = block_on(EchoCompleter.complete(&req)).unwrap();
    assert_eq!(resp.content, Some(String::new()));
}

// ── complete_stream ──────────────────────────────────────────────────────────

#[test]
fn test_complete_stream_echo_yields_one_chunk_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("stream me")]);
    let stream = block_on(EchoCompleter.complete_stream(&req)).unwrap();
    let chunks: Vec<_> = block_on(stream.collect());
    assert_eq!(chunks.len(), 1);
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some("stream me".to_string()));
}

#[test]
fn test_complete_stream_noop_returns_error_error() {
    let req = CompletionRequest::new("x", vec![]);
    let result = block_on(NoopCompleter.complete_stream(&req));
    assert!(matches!(result, Err(CompleteError::ProviderNotFound(_))));
}

#[test]
fn test_complete_stream_empty_messages_yields_empty_delta_edge() {
    let req = CompletionRequest::new("echo", vec![]);
    let stream = block_on(EchoCompleter.complete_stream(&req)).unwrap();
    let chunks: Vec<_> = block_on(stream.collect());
    assert_eq!(chunks.len(), 1);
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some(String::new()));
}

// ── supported_models ─────────────────────────────────────────────────────────

#[test]
fn test_supported_models_echo_contains_echo_happy() {
    assert!(EchoCompleter.supported_models().contains(&"echo".to_string()));
}

#[test]
fn test_supported_models_noop_is_empty_error() {
    assert!(NoopCompleter.supported_models().is_empty());
}

#[test]
fn test_supported_models_returns_owned_strings_edge() {
    let models = EchoCompleter.supported_models();
    assert!(!models.is_empty());
    for m in &models {
        assert!(!m.is_empty());
    }
}

// ── supports ─────────────────────────────────────────────────────────────────

#[test]
fn test_supports_echo_model_returns_true_happy() {
    assert!(EchoCompleter.supports("echo"));
}

#[test]
fn test_supports_unknown_model_returns_false_error() {
    assert!(!EchoCompleter.supports("gpt-999"));
}

#[test]
fn test_supports_empty_string_returns_false_edge() {
    assert!(!EchoCompleter.supports(""));
}

// ── model_info ───────────────────────────────────────────────────────────────

#[test]
fn test_model_info_echo_returns_metadata_happy() {
    let info = block_on(EchoCompleter.model_info("echo")).unwrap();
    assert_eq!(info.id, "echo");
}

#[test]
fn test_model_info_unknown_returns_model_not_found_error() {
    let err = block_on(EchoCompleter.model_info("missing")).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

#[test]
fn test_model_info_noop_always_errors_edge() {
    let err = block_on(NoopCompleter.model_info("echo")).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

// ── list_models ───────────────────────────────────────────────────────────────

#[test]
fn test_list_models_echo_returns_one_entry_happy() {
    let models = block_on(EchoCompleter.list_models()).unwrap();
    assert_eq!(models.len(), 1);
}

#[test]
fn test_list_models_noop_returns_empty_list_error() {
    let models = block_on(NoopCompleter.list_models()).unwrap();
    assert!(models.is_empty());
}

#[test]
fn test_list_models_echo_entries_have_nonempty_ids_edge() {
    let models = block_on(EchoCompleter.list_models()).unwrap();
    for m in &models {
        assert!(!m.id.is_empty());
    }
}

// ── is_model_available ───────────────────────────────────────────────────────

#[test]
fn test_is_model_available_echo_returns_true_happy() {
    assert!(block_on(EchoCompleter.is_model_available("echo")));
}

#[test]
fn test_is_model_available_unknown_returns_false_error() {
    assert!(!block_on(EchoCompleter.is_model_available("ghost")));
}

#[test]
fn test_is_model_available_noop_always_false_edge() {
    assert!(!block_on(NoopCompleter.is_model_available("echo")));
}
