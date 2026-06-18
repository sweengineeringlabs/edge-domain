//! Tests for `FinishReason`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::FinishReason;

/// @covers: FinishReason — variants are distinct
#[test]
fn test_finish_reason_variants_distinct_happy() {
    assert_ne!(FinishReason::Stop, FinishReason::Length);
    assert_ne!(FinishReason::ToolCalls, FinishReason::Error);
}

/// @covers: FinishReason — equality holds for same variant
#[test]
fn test_finish_reason_equality_error() {
    assert_eq!(FinishReason::Stop, FinishReason::Stop);
}

/// @covers: FinishReason — serializes and deserializes correctly
#[test]
fn test_finish_reason_serde_roundtrip_edge() {
    let json = serde_json::to_string(&FinishReason::ContentFilter).expect("serialize");
    let back: FinishReason = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, FinishReason::ContentFilter);
}
