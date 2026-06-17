//! Tests for the `ThinkingProcess` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{ReasoningStep, ThinkingProcess};

fn step(confidence: f32) -> ReasoningStep {
    ReasoningStep::new(0, "x".to_string(), "analysis".to_string())
        .with_confidence(confidence)
        .with_tokens(10)
}

/// @covers: ThinkingProcess::add_step — accumulates tokens and steps
#[test]
fn test_add_step_accumulates_tokens() {
    let mut p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    p.add_step(step(0.9));
    assert_eq!(p.total_tokens, 10);
    assert_eq!(p.step_count(), 1);
}

/// @covers: ThinkingProcess::average_confidence — empty process is zero
#[test]
fn test_average_confidence_empty_is_zero() {
    let p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    assert!((p.average_confidence() - 0.0).abs() < 0.001);
}

/// @covers: ThinkingProcess::average_confidence — averages step confidences
#[test]
fn test_average_confidence_averages_steps() {
    let mut p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    p.add_step(step(0.6));
    p.add_step(step(0.8));
    assert!((p.average_confidence() - 0.7).abs() < 0.001);
}

/// @covers: ThinkingProcess::complete — sets conclusion and completion flag
#[test]
fn test_complete_sets_conclusion() {
    let p =
        ThinkingProcess::new("p".to_string(), "solve".to_string()).complete("answer".to_string());
    assert!(p.is_complete);
    assert_eq!(p.conclusion.as_deref(), Some("answer"));
}

/// @covers: ThinkingProcess::confident_steps — filters high-confidence steps
#[test]
fn test_confident_steps_filters() {
    let mut p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    p.add_step(step(0.9));
    p.add_step(step(0.5));
    assert_eq!(p.confident_steps().len(), 1);
}

/// @covers: ThinkingProcess — serde round-trip
#[test]
fn test_thinking_process_serde_roundtrip() {
    let p = ThinkingProcess::new("p".to_string(), "solve".to_string());
    let json = serde_json::to_string(&p).expect("serialize");
    let back: ThinkingProcess = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.problem, "solve");
}
