//! Tests for the `BufferedStreamHandler` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    AccumulateRequest, BufferedStreamHandler, NextChunkRequest, PendingToolCallRequest,
    StreamDelta, StreamHandler, ToolCallDelta,
};

/// @covers: BufferedStreamHandler::new — starts with no chunks queued
#[test]
fn test_buffered_stream_handler_starts_empty() {
    let mut handler = BufferedStreamHandler::new();
    let response = handler.next_chunk(NextChunkRequest).expect("next_chunk ok");
    assert!(response.chunk.is_none());
}

/// @covers: BufferedStreamHandler — accumulates text into a chunk
#[test]
fn test_buffered_stream_handler_accumulates_text() {
    let mut handler = BufferedStreamHandler::new();
    handler
        .accumulate(AccumulateRequest {
            delta: StreamDelta::text("hi".to_string()),
        })
        .expect("accumulate ok");
    let chunk = handler
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk;
    assert!(
        chunk.is_some(),
        "handler should accumulate text into a chunk"
    );
    let chunk = chunk.unwrap();
    match chunk.delta {
        StreamDelta::Text(ref text) => {
            assert_eq!(text, "hi", "accumulated text should match input")
        }
        ref other => panic!("expected StreamDelta::Text, got {other:?}"),
    }
}

/// @covers: BufferedStreamHandler — tracks a pending tool call
#[test]
fn test_buffered_stream_handler_tracks_pending_call() {
    let mut handler = BufferedStreamHandler::new();
    handler
        .accumulate(AccumulateRequest {
            delta: StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]),
        })
        .expect("accumulate ok");
    let pending = handler
        .pending_tool_call(PendingToolCallRequest)
        .expect("pending_tool_call ok")
        .tool_call;
    assert!(pending.is_some(), "handler should track pending tool call");
    assert_eq!(
        pending.unwrap().index,
        0,
        "pending tool call index should match input"
    );
}
