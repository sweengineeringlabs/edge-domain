//! Scenario coverage for the `stream_ops_svc` SAF surface.

use edge_llm_complete::{NoopCompleter, StreamChunk, StreamDelta, StreamOps, STREAM_OPS_SVC};

#[test]
fn test_stream_ops_svc_constant_is_expected_value_happy() {
    assert_eq!(STREAM_OPS_SVC, "stream_ops");
}

#[test]
fn test_stream_ops_svc_constant_is_nonempty_error() {
    assert!(!STREAM_OPS_SVC.is_empty());
}

#[test]
fn test_stream_ops_apply_delta_updates_chunk_edge() {
    let mut chunk = StreamChunk::partial("x", StreamDelta::empty());
    let delta = StreamDelta::text("update");
    NoopCompleter.apply_delta(&mut chunk, &delta).unwrap();
    assert_eq!(chunk.delta.content, Some("update".to_string()));
}
