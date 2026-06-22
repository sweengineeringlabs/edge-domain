//! Scenario coverage for the `CompleterHandler` marker trait.

use edge_llm_complete::{Completer, CompleterHandler, EchoCompleter};
use futures::executor::block_on;

#[test]
fn test_completer_handler_echo_supports_echo_model_happy() {
    let h: &dyn CompleterHandler = &EchoCompleter;
    assert!(h.supported_models().contains(&"echo".to_string()));
}

#[test]
fn test_completer_handler_echo_complete_stream_returns_ok_error() {
    use edge_llm_complete::{CompletionRequest, Message};
    use futures::StreamExt;
    let req = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let stream = block_on(EchoCompleter.complete_stream(&req)).unwrap();
    let chunks: Vec<_> = block_on(stream.collect());
    assert_eq!(chunks.len(), 1);
}

#[test]
fn test_completer_handler_is_object_safe_edge() {
    fn accepts_handler(_: &dyn CompleterHandler) {}
    accepts_handler(&EchoCompleter);
}
