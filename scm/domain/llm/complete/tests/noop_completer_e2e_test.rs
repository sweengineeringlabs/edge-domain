//! Scenario coverage for `NoopCompleter`.

use edge_llm_complete::{
    CompleteError, CompleteOps, Completer, ContentFlattener, MessageContent, NoopCompleter,
    StreamChunk, StreamDelta, StreamOps, ToolOps,
};
use futures::executor::block_on;

#[test]
fn test_noop_completer_complete_returns_provider_not_found_happy() {
    use edge_llm_complete::CompletionRequest;
    let req = CompletionRequest::new("gpt-4", vec![]);
    let err = block_on(NoopCompleter.complete(&req)).unwrap_err();
    assert!(matches!(err, CompleteError::ProviderNotFound(_)));
}

#[test]
fn test_noop_completer_check_empty_model_returns_error_error() {
    use edge_llm_complete::CompletionRequest;
    let req = CompletionRequest::new("", vec![]);
    assert!(NoopCompleter.check(&req).is_err());
}

#[test]
fn test_noop_completer_flatten_text_content_edge() {
    let content = MessageContent::Text("hi".to_string());
    assert_eq!(NoopCompleter.flatten(&content), "hi");
}

#[test]
fn test_noop_completer_apply_delta_updates_chunk_happy() {
    let mut chunk = StreamChunk::partial("id", StreamDelta::empty());
    NoopCompleter.apply_delta(&mut chunk, &StreamDelta::text("x")).unwrap();
    assert_eq!(chunk.delta.content, Some("x".to_string()));
}

#[test]
fn test_noop_completer_available_tools_is_empty_error() {
    assert!(NoopCompleter.available_tools().is_empty());
}

#[test]
fn test_noop_completer_list_models_returns_empty_edge() {
    let models = block_on(NoopCompleter.list_models()).unwrap();
    assert!(models.is_empty());
}
