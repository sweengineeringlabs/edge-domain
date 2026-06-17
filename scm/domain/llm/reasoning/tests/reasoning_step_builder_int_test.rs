//! Tests for the `ReasoningStepBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::ReasoningStepBuilder;

/// @covers: ReasoningStepBuilder — builds with overrides
#[test]
fn test_reasoning_step_builder_builds_with_overrides() {
    let step = ReasoningStepBuilder::new(2)
        .content("c".to_string())
        .step_type("hypothesis".to_string())
        .confidence(0.9)
        .build();
    assert_eq!(step.index, 2);
    assert!(step.is_confident());
}

/// @covers: ReasoningStepBuilder — clamps out-of-range confidence
#[test]
fn test_reasoning_step_builder_clamps_confidence() {
    let step = ReasoningStepBuilder::new(0).confidence(-3.0).build();
    assert!((step.confidence - 0.0).abs() < 0.001);
}

/// @covers: ReasoningStepBuilder — parent and child wiring
#[test]
fn test_reasoning_step_builder_parent_and_children() {
    let step = ReasoningStepBuilder::new(1)
        .parent_step(0)
        .child_step(2)
        .child_step(2)
        .build();
    assert_eq!(step.parent_step, Some(0));
    assert_eq!(step.child_steps.len(), 1);
}
