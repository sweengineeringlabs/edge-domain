//! Tests for the `ReasoningStep` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::ReasoningStep;

fn step() -> ReasoningStep {
    ReasoningStep::new(0, "thinking".to_string(), "analysis".to_string())
}

/// @covers: ReasoningStep::new — sets core fields with defaults
#[test]
fn test_new_sets_core_fields() {
    let s = step();
    assert_eq!(s.index, 0);
    assert_eq!(s.content, "thinking");
    assert!((s.confidence - 0.5).abs() < 0.001);
}

/// @covers: ReasoningStep::is_confident — high confidence is confident
#[test]
fn test_is_confident_high() {
    assert!(step().with_confidence(0.9).is_confident());
}

/// @covers: ReasoningStep::is_confident — boundary 0.8 is not confident
#[test]
fn test_is_confident_boundary_excludes_point_eight() {
    assert!(!step().with_confidence(0.8).is_confident());
}

/// @covers: ReasoningStep::is_leaf — no children is a leaf
#[test]
fn test_is_leaf_with_no_children() {
    assert!(step().is_leaf());
}

/// @covers: ReasoningStep::add_child — dedupes child indices
#[test]
fn test_add_child_dedupes() {
    let mut s = step();
    s.add_child(1);
    s.add_child(1);
    assert_eq!(s.child_steps.len(), 1);
}

/// @covers: ReasoningStep — serde round-trip
#[test]
fn test_reasoning_step_serde_roundtrip() {
    let json = serde_json::to_string(&step()).expect("serialize");
    let back: ReasoningStep = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.content, "thinking");
}
