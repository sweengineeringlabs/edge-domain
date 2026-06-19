use edge_llm_complete::{FinishReason, StreamChunk, StreamDelta};

#[test]
fn test_partial_chunk_has_no_finish_reason() {
    let chunk = StreamChunk::partial("c-1", StreamDelta::text("hello"));
    assert!(chunk.finish_reason.is_none());
}

#[test]
fn test_terminal_chunk_carries_finish_reason() {
    let chunk = StreamChunk::terminal("c-1", StreamDelta::empty(), FinishReason::Stop);
    assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
}

#[test]
fn test_roundtrip_serialization() {
    let chunk = StreamChunk::terminal("x", StreamDelta::text("hi"), FinishReason::Length);
    let json = serde_json::to_string(&chunk).unwrap();
    let back: StreamChunk = serde_json::from_str(&json).unwrap();
    assert_eq!(chunk, back);
}
