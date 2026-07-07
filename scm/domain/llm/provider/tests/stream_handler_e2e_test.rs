//! SAF facade tests — `StreamHandler` trait via `BufferedStreamHandler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    AccumulateRequest, BufferedStreamHandler, NextChunkRequest, PendingToolCallRequest,
    StreamDelta, StreamHandler, ToolCallDelta,
};

fn handler() -> impl StreamHandler {
    BufferedStreamHandler::new()
}

// --- accumulate ---

/// @covers: StreamHandler::accumulate — text deltas append in order
#[test]
fn test_accumulate_appends_text_happy() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::text("hello ".to_string()),
    })
    .expect("accumulate ok");
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::text("world".to_string()),
    })
    .expect("accumulate ok");
    let chunk = h
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk
        .expect("chunk");
    assert!(matches!(chunk.delta, StreamDelta::Text(ref s) if s == "hello "));
}

/// @covers: StreamHandler::accumulate — empty delta still produces a chunk
#[test]
fn test_accumulate_empty_delta_error() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::empty(),
    })
    .expect("accumulate ok");
    assert!(h
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk
        .is_some());
}

/// @covers: StreamHandler::accumulate — tool-call delta sets pending state
#[test]
fn test_accumulate_tool_call_sets_pending_edge() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]),
    })
    .expect("accumulate ok");
    assert!(h
        .pending_tool_call(PendingToolCallRequest)
        .expect("pending_tool_call ok")
        .tool_call
        .is_some());
}

// --- next_chunk ---

/// @covers: StreamHandler::next_chunk — yields a queued chunk
#[test]
fn test_next_chunk_yields_queued_happy() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::text("hi".to_string()),
    })
    .expect("accumulate ok");
    assert!(h
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk
        .is_some());
}

/// @covers: StreamHandler::next_chunk — empty handler yields nothing
#[test]
fn test_next_chunk_empty_returns_none_error() {
    let mut h = handler();
    assert!(h
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk
        .is_none());
}

/// @covers: StreamHandler::next_chunk — draining leaves the queue empty
#[test]
fn test_next_chunk_drains_to_none_edge() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::text("hi".to_string()),
    })
    .expect("accumulate ok");
    let _ = h.next_chunk(NextChunkRequest);
    assert!(h
        .next_chunk(NextChunkRequest)
        .expect("next_chunk ok")
        .chunk
        .is_none());
}

// --- pending_tool_call ---

/// @covers: StreamHandler::pending_tool_call — reflects an accumulated call
#[test]
fn test_pending_tool_call_present_after_accumulate_happy() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::tool_calls(vec![ToolCallDelta::new(2)]),
    })
    .expect("accumulate ok");
    assert_eq!(
        h.pending_tool_call(PendingToolCallRequest)
            .expect("pending_tool_call ok")
            .tool_call
            .map(|c| c.index),
        Some(2)
    );
}

/// @covers: StreamHandler::pending_tool_call — none before any tool delta
#[test]
fn test_pending_tool_call_none_initially_error() {
    let h = handler();
    assert!(h
        .pending_tool_call(PendingToolCallRequest)
        .expect("pending_tool_call ok")
        .tool_call
        .is_none());
}

/// @covers: StreamHandler::pending_tool_call — text-only stream has no pending call
#[test]
fn test_pending_tool_call_text_only_none_edge() {
    let mut h = handler();
    h.accumulate(AccumulateRequest {
        delta: StreamDelta::text("hi".to_string()),
    })
    .expect("accumulate ok");
    assert!(h
        .pending_tool_call(PendingToolCallRequest)
        .expect("pending_tool_call ok")
        .tool_call
        .is_none());
}
