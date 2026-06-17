//! Tests for the `StepResultBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::StepResultBuilder;

/// @covers: StepResultBuilder — default builds a successful result
#[test]
fn test_step_result_builder_default_success() {
    assert!(StepResultBuilder::new().build().success);
}

/// @covers: StepResultBuilder — error sets failure and stops continuation
#[test]
fn test_step_result_builder_error_stops_continuation() {
    let result = StepResultBuilder::new().error("boom".to_string()).build();
    assert!(!result.success);
    assert!(!result.should_continue);
}

/// @covers: StepResultBuilder — carries output and next action
#[test]
fn test_step_result_builder_output_and_next_action() {
    let result = StepResultBuilder::new()
        .output("out".to_string())
        .next_action("retry".to_string())
        .build();
    assert_eq!(result.output, "out");
    assert_eq!(result.next_action.as_deref(), Some("retry"));
}
