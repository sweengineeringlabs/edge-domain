//! Tests for the `BufferedStreamHandler` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{BufferedStreamHandler, StreamDelta, StreamHandler, ToolCallDelta};

/// @covers: BufferedStreamHandler::new — starts with no chunks queued
#[test]
fn test_buffered_stream_handler_starts_empty() {
    let mut handler = BufferedStreamHandler::new();
    assert!(handler.next_chunk().is_none());
}

/// @covers: BufferedStreamHandler — accumulates text into a chunk
#[test]
fn test_buffered_stream_handler_accumulates_text() {
    let mut handler = BufferedStreamHandler::new();
    handler.accumulate(StreamDelta::text("hi".to_string()));
    let chunk = handler.next_chunk();
    assert!(chunk.is_some(), "handler should accumulate text into a chunk");
    assert_eq!(chunk.unwrap().text, "hi", "accumulated text should match input");
}

/// @covers: BufferedStreamHandler — tracks a pending tool call
#[test]
fn test_buffered_stream_handler_tracks_pending_call() {
    let mut handler = BufferedStreamHandler::new();
    handler.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]));
    let pending = handler.pending_tool_call();
    assert!(pending.is_some(), "handler should track pending tool call");
    assert_eq!(pending.unwrap().id, 0, "pending tool call id should match input");
}
