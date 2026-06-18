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
    assert!(handler.next_chunk().is_some());
}

/// @covers: BufferedStreamHandler — tracks a pending tool call
#[test]
fn test_buffered_stream_handler_tracks_pending_call() {
    let mut handler = BufferedStreamHandler::new();
    handler.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]));
    assert!(handler.pending_tool_call().is_some());
}
