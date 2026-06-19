//! Scenario coverage for the `StreamOps` trait.

use edge_llm_complete::{CompleteError, FinishReason, StreamChunk, StreamDelta, StreamOps};

struct AccumulatorOps;

impl StreamOps for AccumulatorOps {
    fn apply_delta(&self, chunk: &mut StreamChunk, delta: &StreamDelta) -> Result<(), CompleteError> {
        match (&mut chunk.delta.content, delta.content.clone()) {
            (Some(existing), Some(new)) => existing.push_str(&new),
            (slot @ None, Some(new)) => *slot = Some(new),
            _ => {}
        }
        Ok(())
    }
}

// ── apply_delta ───────────────────────────────────────────────────────────────

#[test]
fn test_apply_delta_appends_content_happy() {
    let mut chunk = StreamChunk::partial("id-1", StreamDelta::text("hello"));
    let delta = StreamDelta::text(" world");
    AccumulatorOps.apply_delta(&mut chunk, &delta).unwrap();
    assert_eq!(chunk.delta.content, Some("hello world".to_string()));
}

#[test]
fn test_apply_delta_empty_delta_is_noop_error() {
    let mut chunk = StreamChunk::partial("id-1", StreamDelta::text("hi"));
    AccumulatorOps.apply_delta(&mut chunk, &StreamDelta::empty()).unwrap();
    assert_eq!(chunk.delta.content, Some("hi".to_string()));
}

#[test]
fn test_apply_delta_initialises_none_content_edge() {
    let mut chunk = StreamChunk::partial("id-1", StreamDelta::empty());
    AccumulatorOps.apply_delta(&mut chunk, &StreamDelta::text("init")).unwrap();
    assert_eq!(chunk.delta.content, Some("init".to_string()));
}

// ── into_chunk ────────────────────────────────────────────────────────────────

#[test]
fn test_into_chunk_sets_id_and_delta_happy() {
    let delta = StreamDelta::text("hello");
    let chunk = AccumulatorOps::into_chunk("c-1".to_string(), delta);
    assert_eq!(chunk.id, "c-1");
    assert_eq!(chunk.delta.content, Some("hello".to_string()));
}

#[test]
fn test_into_chunk_finish_reason_is_none_error() {
    let chunk = AccumulatorOps::into_chunk("c-1".to_string(), StreamDelta::empty());
    assert!(chunk.finish_reason.is_none());
}

#[test]
fn test_into_chunk_empty_id_is_valid_edge() {
    let chunk = AccumulatorOps::into_chunk(String::new(), StreamDelta::text("x"));
    assert!(chunk.id.is_empty());
    assert_eq!(chunk.finish_reason, None);
    let _ = FinishReason::Stop; // ensure FinishReason is exercised in this module
}
