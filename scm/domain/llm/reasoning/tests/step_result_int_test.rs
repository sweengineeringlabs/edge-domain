//! Tests for the `StepResult` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::StepResult;

/// @covers: StepResult::success — marks the result successful
#[test]
fn test_success_is_successful() {
    assert!(StepResult::success("out".to_string()).success);
}

/// @covers: StepResult::failure — carries the error message
#[test]
fn test_failure_carries_error() {
    let result = StepResult::failure("boom".to_string());
    assert_eq!(result.error.as_deref(), Some("boom"));
}

/// @covers: StepResult::was_fast — sub-second is fast
#[test]
fn test_was_fast_under_one_second() {
    assert!(StepResult::success("x".to_string())
        .with_duration(500)
        .was_fast());
}

/// @covers: StepResult::was_fast — one second is not fast
#[test]
fn test_was_fast_at_one_second_is_slow() {
    assert!(!StepResult::success("x".to_string())
        .with_duration(1000)
        .was_fast());
}

/// @covers: StepResult — serde round-trip
#[test]
fn test_step_result_serde_roundtrip() {
    let result = StepResult::success("done".to_string()).with_next_action("retry".to_string());
    let json = serde_json::to_string(&result).expect("serialize");
    let back: StepResult = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.next_action.as_deref(), Some("retry"));
}
