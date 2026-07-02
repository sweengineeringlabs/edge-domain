//! Scenario coverage for `NoopCompleter`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    AvailableToolsRequest, CompleteError, CompleteOps, CompleteRequest, Completer,
    CompletionCheckRequest, ContentFlattener, DeltaApplicationRequest, FlattenRequest,
    ListModelsRequest, MessageContent, NoopCompleter, StreamChunk, StreamDelta, StreamOps, ToolOps,
};
use futures::executor::block_on;

#[test]
fn test_noop_completer_complete_returns_provider_not_found_happy() {
    use edge_llm_complete::CompletionRequest;
    let req = CompletionRequest::new("gpt-4", vec![]);
    let err = block_on(NoopCompleter.complete(CompleteRequest { request: &req })).unwrap_err();
    assert!(matches!(err, CompleteError::ProviderNotFound(_)));
}

#[test]
fn test_noop_completer_check_empty_model_returns_error_error() {
    use edge_llm_complete::CompletionRequest;
    let req = CompletionRequest::new("", vec![]);
    assert!(NoopCompleter
        .check(CompletionCheckRequest { request: &req })
        .is_err());
}

#[test]
fn test_noop_completer_flatten_text_content_edge() {
    let content = MessageContent::Text("hi".to_string());
    let resp = NoopCompleter
        .flatten(FlattenRequest { content: &content })
        .unwrap();
    assert_eq!(resp.text, "hi");
}

#[test]
fn test_noop_completer_apply_delta_updates_chunk_happy() {
    let mut chunk = StreamChunk::partial("id", StreamDelta::empty());
    NoopCompleter
        .apply_delta(DeltaApplicationRequest {
            chunk: &mut chunk,
            delta: &StreamDelta::text("x"),
        })
        .unwrap();
    assert_eq!(chunk.delta.content, Some("x".to_string()));
}

#[test]
fn test_noop_completer_available_tools_is_empty_error() {
    assert!(NoopCompleter
        .available_tools(AvailableToolsRequest)
        .unwrap()
        .tools
        .is_empty());
}

#[test]
fn test_noop_completer_list_models_returns_empty_edge() {
    let models = block_on(NoopCompleter.list_models(ListModelsRequest))
        .unwrap()
        .models;
    assert!(models.is_empty());
}
