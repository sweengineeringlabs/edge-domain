//! Scenario coverage for the `CompletionStream` type alias.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    Completer, CompletionRequest, CompletionStream, CompletionStreamRequest, EchoCompleter, Message,
};
use futures::executor::block_on;
use futures::StreamExt;

#[test]
fn test_completion_stream_echo_yields_chunks_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("ping")]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let stream: CompletionStream = resp.stream;
    let chunks: Vec<_> = block_on(stream.collect());
    assert!(!chunks.is_empty());
}

#[test]
fn test_completion_stream_chunk_content_matches_input_error() {
    let req = CompletionRequest::new("echo", vec![Message::user("expected")]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let stream: CompletionStream = resp.stream;
    let chunks: Vec<_> = block_on(stream.collect());
    let chunk = chunks.into_iter().next().unwrap().unwrap();
    assert_eq!(chunk.delta.content, Some("expected".to_string()));
}

#[test]
fn test_completion_stream_is_sendable_edge() {
    let req = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let resp =
        block_on(EchoCompleter.complete_stream(CompletionStreamRequest { request: &req })).unwrap();
    let stream: CompletionStream = resp.stream;
    // Moving the stream into a spawned thread only compiles if CompletionStream: Send.
    let handle = std::thread::spawn(move || block_on(stream.collect::<Vec<_>>()));
    let chunks = handle.join().expect("thread panicked");
    assert!(!chunks.is_empty());
}
