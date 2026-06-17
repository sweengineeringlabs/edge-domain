//! Tests for the `ReasoningPattern` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::ReasoningPattern;

/// @covers: ReasoningPattern::is_iterative — reflection iterates
#[test]
fn test_is_iterative_reflection() {
    assert!(ReasoningPattern::Reflection.is_iterative());
}

/// @covers: ReasoningPattern::is_iterative — chain-of-thought does not
#[test]
fn test_is_iterative_chain_of_thought_false() {
    assert!(!ReasoningPattern::ChainOfThought.is_iterative());
}

/// @covers: ReasoningPattern::is_collaborative — multi-agent collaborates
#[test]
fn test_is_collaborative_multi_agent() {
    assert!(ReasoningPattern::MultiAgent.is_collaborative());
}

/// @covers: ReasoningPattern::as_str — human-readable label
#[test]
fn test_as_str_labels_pattern() {
    assert_eq!(
        ReasoningPattern::ChainOfThought.as_str(),
        "Chain of Thought"
    );
}

/// @covers: ReasoningPattern::expected_step_count — positive estimate
#[test]
fn test_expected_step_count_positive() {
    assert!(ReasoningPattern::GraphBased.expected_step_count() > 0);
}

/// @covers: ReasoningPattern — serde round-trip
#[test]
fn test_reasoning_pattern_serde_roundtrip() {
    let json = serde_json::to_string(&ReasoningPattern::TreeOfThought).expect("serialize");
    let back: ReasoningPattern = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ReasoningPattern::TreeOfThought);
}
