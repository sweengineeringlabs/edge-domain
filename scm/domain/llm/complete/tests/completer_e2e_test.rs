//! Scenario coverage for the `Completer` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    CompleteError, CompleteRequest, Completer, CompleterHealthCheckRequest, CompletionRequest,
    CompletionStreamRequest, EchoCompleter, ListModelsRequest, Message, ModelAvailabilityRequest,
    ModelInfoRequest, ModelSupportRequest, NoopCompleter, SupportedModelsRequest,
};
use futures::executor::block_on;
use futures::StreamExt;

// ── complete ────────────────────────────────────────────────────────────────

#[test]
fn test_complete_echo_returns_user_content_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("hello")]);
    let resp = block_on(EchoCompleter.complete(CompleteRequest { request: &req })).unwrap();
    assert_eq!(resp.content, Some("hello".to_string()));
}

#[test]
fn test_complete_noop_returns_provider_not_found_error() {
    let req = CompletionRequest::new("x", vec![]);
    let err = block_on(NoopCompleter.complete(CompleteRequest { request: &req })).unwrap_err();
    assert!(matches!(err, CompleteError::ProviderNotFound(_)));
}

#[test]
fn test_complete_empty_messages_returns_empty_content_edge() {
    let req = CompletionRequest::new("echo", vec![]);
    let resp = block_on(EchoCompleter.complete(CompleteRequest { request: &req })).unwrap();
    assert_eq!(resp.content, Some(String::new()));
}

// ── complete_stream ──────────────────────────────────────────────────────────

#[test]
fn test_complete_stream_echo_yields_one_chunk_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("stream me")]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let chunks: Vec<_> = block_on(resp.stream.collect());
    assert_eq!(chunks.len(), 1);
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some("stream me".to_string()));
}

#[test]
fn test_complete_stream_noop_returns_error_error() {
    let req = CompletionRequest::new("x", vec![]);
    let result = block_on(NoopCompleter.complete_stream(CompletionStreamRequest { request: &req }));
    assert!(matches!(result, Err(CompleteError::ProviderNotFound(_))));
}

#[test]
fn test_complete_stream_empty_messages_yields_empty_delta_edge() {
    let req = CompletionRequest::new("echo", vec![]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let chunks: Vec<_> = block_on(resp.stream.collect());
    assert_eq!(chunks.len(), 1);
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some(String::new()));
}

// ── supported_models ─────────────────────────────────────────────────────────

#[test]
fn test_supported_models_echo_contains_echo_happy() {
    assert!(EchoCompleter
        .supported_models(SupportedModelsRequest)
        .unwrap()
        .models
        .contains(&"echo".to_string()));
}

#[test]
fn test_supported_models_noop_is_empty_error() {
    assert!(NoopCompleter
        .supported_models(SupportedModelsRequest)
        .unwrap()
        .models
        .is_empty());
}

#[test]
fn test_supported_models_returns_owned_strings_edge() {
    let models = EchoCompleter
        .supported_models(SupportedModelsRequest)
        .unwrap()
        .models;
    assert!(!models.is_empty());
    for m in &models {
        assert!(!m.is_empty());
    }
}

// ── supports ─────────────────────────────────────────────────────────────────

#[test]
fn test_supports_echo_model_returns_true_happy() {
    assert!(
        EchoCompleter
            .supports(ModelSupportRequest { model: "echo" })
            .unwrap()
            .supported
    );
}

#[test]
fn test_supports_unknown_model_returns_false_error() {
    assert!(
        !EchoCompleter
            .supports(ModelSupportRequest { model: "gpt-999" })
            .unwrap()
            .supported
    );
}

#[test]
fn test_supports_empty_string_returns_false_edge() {
    assert!(
        !EchoCompleter
            .supports(ModelSupportRequest { model: "" })
            .unwrap()
            .supported
    );
}

// ── model_info ───────────────────────────────────────────────────────────────

#[test]
fn test_model_info_echo_returns_metadata_happy() {
    let info = block_on(EchoCompleter.model_info(ModelInfoRequest { model: "echo" }))
        .unwrap()
        .info;
    assert_eq!(info.id, "echo");
}

#[test]
fn test_model_info_unknown_returns_model_not_found_error() {
    let err =
        block_on(EchoCompleter.model_info(ModelInfoRequest { model: "missing" })).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

#[test]
fn test_model_info_noop_always_errors_edge() {
    let err = block_on(NoopCompleter.model_info(ModelInfoRequest { model: "echo" })).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

// ── list_models ───────────────────────────────────────────────────────────────

#[test]
fn test_list_models_echo_returns_one_entry_happy() {
    let models = block_on(EchoCompleter.list_models(ListModelsRequest))
        .unwrap()
        .models;
    assert_eq!(models.len(), 1);
}

#[test]
fn test_list_models_noop_returns_empty_list_error() {
    let models = block_on(NoopCompleter.list_models(ListModelsRequest))
        .unwrap()
        .models;
    assert!(models.is_empty());
}

#[test]
fn test_list_models_echo_entries_have_nonempty_ids_edge() {
    let models = block_on(EchoCompleter.list_models(ListModelsRequest))
        .unwrap()
        .models;
    for m in &models {
        assert!(!m.id.is_empty());
    }
}

// ── is_model_available ───────────────────────────────────────────────────────

#[test]
fn test_is_model_available_echo_returns_true_happy() {
    assert!(
        block_on(EchoCompleter.is_model_available(ModelAvailabilityRequest { model: "echo" }))
            .unwrap()
            .available
    );
}

#[test]
fn test_is_model_available_unknown_returns_false_error() {
    assert!(
        !block_on(EchoCompleter.is_model_available(ModelAvailabilityRequest { model: "ghost" }))
            .unwrap()
            .available
    );
}

#[test]
fn test_is_model_available_noop_always_false_edge() {
    assert!(
        !block_on(NoopCompleter.is_model_available(ModelAvailabilityRequest { model: "echo" }))
            .unwrap()
            .available
    );
}

// ── health_check ────────────────────────────────────────────────────────────

#[test]
fn test_health_check_echo_returns_true_happy() {
    assert!(
        block_on(EchoCompleter.health_check(CompleterHealthCheckRequest))
            .unwrap()
            .healthy
    );
}

#[test]
fn test_health_check_noop_returns_true_error() {
    assert!(
        block_on(NoopCompleter.health_check(CompleterHealthCheckRequest))
            .unwrap()
            .healthy
    );
}

#[test]
fn test_health_check_delegates_to_list_models_edge() {
    let echo_health = block_on(EchoCompleter.health_check(CompleterHealthCheckRequest))
        .unwrap()
        .healthy;
    let echo_models = block_on(EchoCompleter.list_models(ListModelsRequest));
    assert_eq!(echo_health, echo_models.is_ok());
}
