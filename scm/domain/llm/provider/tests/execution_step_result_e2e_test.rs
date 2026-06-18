//! Tests for `ExecutionStepResult`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionStepResult, TokenUsage};

/// @covers: ExecutionStepResult::has_action — true when action present
#[test]
fn test_has_action_true_with_action_happy() {
    let r = ExecutionStepResult::new("think".to_string(), Some("tool".to_string()), 0.8, Some(TokenUsage::new(0, 0, 0, 0)));
    assert!(r.has_action());
}

/// @covers: ExecutionStepResult::has_action — false when no action
#[test]
fn test_has_action_false_without_action_error() {
    let r = ExecutionStepResult::new("think".to_string(), None, 0.5, Some(TokenUsage::new(0, 0, 0, 0)));
    assert!(!r.has_action());
}

/// @covers: ExecutionStepResult — high confidence above threshold
#[test]
fn test_high_confidence_above_threshold_edge() {
    let r = ExecutionStepResult::new("x".to_string(), None, 0.9, Some(TokenUsage::new(0, 0, 0, 0)));
    assert!(r.high_confidence());
}

/// @covers: ExecutionStepResult — serializes and deserializes correctly
#[test]
fn test_execution_step_result_serde_roundtrip_edge() {
    let r = ExecutionStepResult::new("x".to_string(), None, 0.9, Some(TokenUsage::new(1, 1, 0, 0)));
    let json = serde_json::to_string(&r).expect("serialize");
    let back: ExecutionStepResult = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.confidence, 0.9);
}
