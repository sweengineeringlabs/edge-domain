//! SAF facade tests — `Reasoning` trait via `LinearReasoning`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{
    Reasoning, ReasoningBootstrap, ReasoningPattern, StdReasoningFactory, ThinkingProcess,
};
use futures::executor::block_on;

fn reasoner(pattern: ReasoningPattern) -> impl Reasoning {
    StdReasoningFactory::reasoning(pattern)
}

// --- reason ---

/// @covers: Reasoning::reason — produces a completed process on success
#[test]
fn test_reason_completes_process_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = block_on(r.reason("solve x", ReasoningPattern::ChainOfThought))
        .expect("reasoning should succeed");
    assert!(process.is_complete);
}

/// @covers: Reasoning::reason — rejects an unsupported pattern
#[test]
fn test_reason_unsupported_pattern_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(block_on(r.reason("solve x", ReasoningPattern::GraphBased)).is_err());
}

/// @covers: Reasoning::reason — blank problem is rejected
#[test]
fn test_reason_blank_problem_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(block_on(r.reason("   ", ReasoningPattern::ChainOfThought)).is_err());
}

// --- supported_patterns ---

/// @covers: Reasoning::supported_patterns — lists the configured pattern
#[test]
fn test_supported_patterns_lists_pattern_happy() {
    let r = reasoner(ReasoningPattern::TreeOfThought);
    assert_eq!(
        r.supported_patterns(),
        vec![ReasoningPattern::TreeOfThought]
    );
}

/// @covers: Reasoning::supported_patterns — does not list other patterns
#[test]
fn test_supported_patterns_excludes_others_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(!r
        .supported_patterns()
        .contains(&ReasoningPattern::GraphBased));
}

/// @covers: Reasoning::supported_patterns — stable across calls
#[test]
fn test_supported_patterns_stable_edge() {
    let r = reasoner(ReasoningPattern::Reflection);
    assert_eq!(r.supported_patterns(), r.supported_patterns());
}

// --- supports_pattern ---

/// @covers: Reasoning::supports_pattern — true for the configured pattern
#[test]
fn test_supports_pattern_true_for_configured_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r.supports_pattern(ReasoningPattern::ChainOfThought));
}

/// @covers: Reasoning::supports_pattern — false for a different pattern
#[test]
fn test_supports_pattern_false_for_other_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(!r.supports_pattern(ReasoningPattern::MultiAgent));
}

/// @covers: Reasoning::supports_pattern — consistent with supported_patterns
#[test]
fn test_supports_pattern_matches_list_edge() {
    let r = reasoner(ReasoningPattern::FewShot);
    assert_eq!(
        r.supports_pattern(ReasoningPattern::FewShot),
        r.supported_patterns().contains(&ReasoningPattern::FewShot)
    );
}

// --- pattern_metadata ---

/// @covers: Reasoning::pattern_metadata — returns metadata for a supported pattern
#[test]
fn test_pattern_metadata_returns_some_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let meta = r.pattern_metadata(ReasoningPattern::ChainOfThought);
    assert!(meta.is_some(), "should return metadata for supported pattern");
    assert_eq!(meta.unwrap().pattern, ReasoningPattern::ChainOfThought, "metadata should carry the pattern");
}

/// @covers: Reasoning::pattern_metadata — none for an unsupported pattern
#[test]
fn test_pattern_metadata_none_for_unsupported_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r.pattern_metadata(ReasoningPattern::GraphBased).is_none());
}

/// @covers: Reasoning::pattern_metadata — carries the requested pattern
#[test]
fn test_pattern_metadata_carries_pattern_edge() {
    let r = reasoner(ReasoningPattern::TreeOfThought);
    let meta = r
        .pattern_metadata(ReasoningPattern::TreeOfThought)
        .expect("metadata");
    assert_eq!(meta.pattern, ReasoningPattern::TreeOfThought);
}

// --- validate_problem ---

/// @covers: Reasoning::validate_problem — accepts a non-empty problem
#[test]
fn test_validate_problem_accepts_nonempty_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let result = r.validate_problem("solve x");
    assert_eq!(result, Ok(()), "non-empty problem should be valid");
}

/// @covers: Reasoning::validate_problem — rejects an empty problem
#[test]
fn test_validate_problem_rejects_empty_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r.validate_problem("").is_err());
}

/// @covers: Reasoning::validate_problem — whitespace-only is rejected
#[test]
fn test_validate_problem_rejects_whitespace_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r.validate_problem("   \t ").is_err());
}

// --- next_step ---

/// @covers: Reasoning::next_step — produces a step at the next index
#[test]
fn test_next_step_uses_next_index_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "solve x".to_string());
    assert_eq!(r.next_step(&process).index, 0);
}

/// @covers: Reasoning::next_step — index advances with existing steps
#[test]
fn test_next_step_advances_with_steps_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = block_on(r.reason("solve x", ReasoningPattern::ChainOfThought))
        .expect("reasoning should succeed");
    assert_eq!(r.next_step(&process).index, process.step_count());
}

/// @covers: Reasoning::next_step — references the process problem
#[test]
fn test_next_step_references_problem_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "ship it".to_string());
    assert!(r.next_step(&process).content.contains("ship it"));
}

// --- evaluate_step ---

/// @covers: Reasoning::evaluate_step — succeeds for a non-empty step
#[test]
fn test_evaluate_step_succeeds_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "solve x".to_string());
    let step = r.next_step(&process);
    assert!(r.evaluate_step(&step).success);
}

/// @covers: Reasoning::evaluate_step — fails for an empty-content step
#[test]
fn test_evaluate_step_fails_on_empty_error() {
    use edge_llm_reasoning::ReasoningStep;
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let step = ReasoningStep::new(0, String::new(), "analysis".to_string());
    assert!(!r.evaluate_step(&step).success);
}

/// @covers: Reasoning::evaluate_step — confident step stops continuation
#[test]
fn test_evaluate_step_confident_stops_edge() {
    use edge_llm_reasoning::ReasoningStep;
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let step =
        ReasoningStep::new(0, "done".to_string(), "synthesis".to_string()).with_confidence(0.95);
    assert!(!r.evaluate_step(&step).should_continue);
}

// --- build_chain ---

/// @covers: Reasoning::build_chain — collects processes into a chain
#[test]
fn test_build_chain_collects_processes_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let p = block_on(r.reason("solve x", ReasoningPattern::ChainOfThought))
        .expect("reasoning should succeed");
    let chain = r.build_chain("c1", vec![p]);
    assert_eq!(chain.process_count(), 1);
}

/// @covers: Reasoning::build_chain — empty input yields an empty chain
#[test]
fn test_build_chain_empty_input_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let chain = r.build_chain("c1", vec![]);
    assert_eq!(chain.process_count(), 0);
}

/// @covers: Reasoning::build_chain — preserves the chain id
#[test]
fn test_build_chain_preserves_id_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let chain = r.build_chain("chain-xyz", vec![]);
    assert_eq!(chain.id, "chain-xyz");
}
