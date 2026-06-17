//! Tests for the `ThinkingProcessBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{ReasoningStep, ThinkingProcessBuilder};

/// @covers: ThinkingProcessBuilder — builds with problem and steps
#[test]
fn test_thinking_process_builder_builds_with_steps() {
    let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_tokens(5);
    let process = ThinkingProcessBuilder::new("p1".to_string())
        .problem("solve".to_string())
        .step(step)
        .build();
    assert_eq!(process.step_count(), 1);
    assert_eq!(process.total_tokens, 5);
}

/// @covers: ThinkingProcessBuilder — incomplete without a conclusion
#[test]
fn test_thinking_process_builder_incomplete_default() {
    assert!(
        !ThinkingProcessBuilder::new("p1".to_string())
            .build()
            .is_complete
    );
}

/// @covers: ThinkingProcessBuilder — conclusion completes the process
#[test]
fn test_thinking_process_builder_conclusion_completes() {
    let process = ThinkingProcessBuilder::new("p1".to_string())
        .conclusion("answer".to_string())
        .build();
    assert!(process.is_complete);
}
