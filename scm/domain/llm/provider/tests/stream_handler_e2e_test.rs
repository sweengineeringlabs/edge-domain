//! SAF facade tests — `StreamHandler` trait via `BufferedStreamHandler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    ProviderBootstrap, StdProviderFactory, StreamDelta, StreamHandler, ToolCallDelta,
};

fn handler() -> impl StreamHandler {
    StdProviderFactory::stream_handler()
}

// --- accumulate ---

/// @covers: StreamHandler::accumulate — text deltas append in order
#[test]
fn test_accumulate_appends_text_happy() {
    let mut h = handler();
    h.accumulate(StreamDelta::text("hello ".to_string()));
    h.accumulate(StreamDelta::text("world".to_string()));
    let chunk = h.next_chunk().expect("chunk");
    let _ = h; // handler consumed below via chunk
    assert!(matches!(chunk.delta, StreamDelta::Text(ref s) if s == "hello "));
}

/// @covers: StreamHandler::accumulate — empty delta still produces a chunk
#[test]
fn test_accumulate_empty_delta_error() {
    let mut h = handler();
    h.accumulate(StreamDelta::empty());
    assert!(h.next_chunk().unwrap());
}

/// @covers: StreamHandler::accumulate — tool-call delta sets pending state
#[test]
fn test_accumulate_tool_call_sets_pending_edge() {
    let mut h = handler();
    h.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]));
    assert!(h.pending_tool_call().unwrap());
}

// --- next_chunk ---

/// @covers: StreamHandler::next_chunk — yields a queued chunk
#[test]
fn test_next_chunk_yields_queued_happy() {
    let mut h = handler();
    h.accumulate(StreamDelta::text("hi".to_string()));
    assert!(h.next_chunk().unwrap());
}

/// @covers: StreamHandler::next_chunk — empty handler yields nothing
#[test]
fn test_next_chunk_empty_returns_none_error() {
    let mut h = handler();
    assert!(h.next_chunk().is_none());
}

/// @covers: StreamHandler::next_chunk — draining leaves the queue empty
#[test]
fn test_next_chunk_drains_to_none_edge() {
    let mut h = handler();
    h.accumulate(StreamDelta::text("hi".to_string()));
    let _ = h.next_chunk();
    assert!(h.next_chunk().is_none());
}

// --- pending_tool_call ---

/// @covers: StreamHandler::pending_tool_call — reflects an accumulated call
#[test]
fn test_pending_tool_call_present_after_accumulate_happy() {
    let mut h = handler();
    h.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(2)]));
    assert_eq!(h.pending_tool_call().map(|c| c.index), Some(2));
}

/// @covers: StreamHandler::pending_tool_call — none before any tool delta
#[test]
fn test_pending_tool_call_none_initially_error() {
    let h = handler();
    assert!(h.pending_tool_call().is_none());
}

/// @covers: StreamHandler::pending_tool_call — text-only stream has no pending call
#[test]
fn test_pending_tool_call_text_only_none_edge() {
    let mut h = handler();
    h.accumulate(StreamDelta::text("hi".to_string()));
    assert!(h.pending_tool_call().is_none());
}
