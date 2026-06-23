//! Scenario coverage for the `CompletionStream` type alias.

use edge_llm_complete::{Completer, CompletionRequest, CompletionStream, EchoCompleter, Message};
use futures::executor::block_on;
use futures::StreamExt;

#[test]
fn test_completion_stream_echo_yields_chunks_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("ping")]);
    let stream: CompletionStream = block_on(EchoCompleter.complete_stream(&req)).unwrap();
    let chunks: Vec<_> = block_on(stream.collect());
    assert!(!chunks.is_empty());
}

#[test]
fn test_completion_stream_chunk_content_matches_input_error() {
    let req = CompletionRequest::new("echo", vec![Message::user("expected")]);
    let stream: CompletionStream = block_on(EchoCompleter.complete_stream(&req)).unwrap();
    let chunks: Vec<_> = block_on(stream.collect());
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some("expected".to_string()));
}

#[test]
fn test_completion_stream_is_sendable_edge() {
    fn assert_send<T: Send>() {}
    assert_send::<CompletionStream>();
    // If this test compiles without error, CompletionStream implements Send
    assert!(std::marker::PhantomData::<CompletionStream> != std::marker::PhantomData::<()>);
}
