//! Scenario coverage for the `CompleterHandler` marker trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{Completer, CompleterHandler, EchoCompleter, SupportedModelsRequest};
use futures::executor::block_on;

#[test]
fn test_completer_handler_echo_supports_echo_model_happy() {
    let h: &dyn CompleterHandler = &EchoCompleter;
    assert!(h
        .supported_models(SupportedModelsRequest)
        .unwrap()
        .models
        .contains(&"echo".to_string()));
}

#[test]
fn test_completer_handler_echo_complete_stream_returns_ok_error() {
    use edge_llm_complete::{CompletionRequest, CompletionStreamRequest, Message};
    use futures::StreamExt;
    let req = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let chunks: Vec<_> = block_on(resp.stream.collect());
    assert_eq!(chunks.len(), 1);
}

#[test]
fn test_completer_handler_is_object_safe_edge() {
    fn accepts_handler(_: &dyn CompleterHandler) {}
    accepts_handler(&EchoCompleter);
}
