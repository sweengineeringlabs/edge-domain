//! SAF facade tests — `Reasoning` trait via `LinearReasoning`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{
    ChainBuildRequest, NextStepRequest, PatternMetadataLookupRequest, PatternSupportRequest,
    ProblemValidationRequest, ReasonRequest, Reasoning, ReasoningBootstrap, ReasoningPattern,
    StdReasoningFactory, StepEvaluationRequest, SupportedPatternsRequest, ThinkingProcess,
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
    let resp = block_on(r.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .expect("reasoning should succeed");
    assert!(resp.process.is_complete);
}

/// @covers: Reasoning::reason — rejects an unsupported pattern
#[test]
fn test_reason_unsupported_pattern_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(block_on(r.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::GraphBased,
    }))
    .is_err());
}

/// @covers: Reasoning::reason — blank problem is rejected
#[test]
fn test_reason_blank_problem_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(block_on(r.reason(ReasonRequest {
        problem: "   ",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .is_err());
}

// --- supported_patterns ---

/// @covers: Reasoning::supported_patterns — lists the configured pattern
#[test]
fn test_supported_patterns_lists_pattern_happy() {
    let r = reasoner(ReasoningPattern::TreeOfThought);
    let resp = r
        .supported_patterns(SupportedPatternsRequest)
        .expect("supported_patterns should succeed");
    assert_eq!(resp.patterns, vec![ReasoningPattern::TreeOfThought]);
}

/// @covers: Reasoning::supported_patterns — does not list other patterns
#[test]
fn test_supported_patterns_excludes_others_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .supported_patterns(SupportedPatternsRequest)
        .expect("supported_patterns should succeed");
    assert!(!resp.patterns.contains(&ReasoningPattern::GraphBased));
}

/// @covers: Reasoning::supported_patterns — contains the configured pattern
#[test]
fn test_supported_patterns_contains_configured_edge() {
    let r = reasoner(ReasoningPattern::Reflection);
    let resp = r
        .supported_patterns(SupportedPatternsRequest)
        .expect("supported_patterns should succeed");
    assert!(
        resp.patterns.contains(&ReasoningPattern::Reflection),
        "supported_patterns should include the configured pattern"
    );
}

// --- supports_pattern ---

/// @covers: Reasoning::supports_pattern — true for the configured pattern
#[test]
fn test_supports_pattern_true_for_configured_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::ChainOfThought,
        })
        .expect("supports_pattern should succeed");
    assert!(resp.supported);
}

/// @covers: Reasoning::supports_pattern — false for a different pattern
#[test]
fn test_supports_pattern_false_for_other_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::MultiAgent,
        })
        .expect("supports_pattern should succeed");
    assert!(!resp.supported);
}

/// @covers: Reasoning::supports_pattern — consistent with supported_patterns
#[test]
fn test_supports_pattern_matches_list_edge() {
    let r = reasoner(ReasoningPattern::FewShot);
    let supports_resp = r
        .supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::FewShot,
        })
        .expect("supports_pattern should succeed");
    let list_resp = r
        .supported_patterns(SupportedPatternsRequest)
        .expect("supported_patterns should succeed");
    assert_eq!(
        supports_resp.supported,
        list_resp.patterns.contains(&ReasoningPattern::FewShot)
    );
}

// --- pattern_metadata ---

/// @covers: Reasoning::pattern_metadata — returns metadata for a supported pattern
#[test]
fn test_pattern_metadata_returns_some_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .pattern_metadata(PatternMetadataLookupRequest {
            pattern: ReasoningPattern::ChainOfThought,
        })
        .expect("pattern_metadata should succeed");
    assert!(
        resp.metadata.is_some(),
        "should return metadata for supported pattern"
    );
    assert_eq!(
        resp.metadata.unwrap().pattern,
        ReasoningPattern::ChainOfThought,
        "metadata should carry the pattern"
    );
}

/// @covers: Reasoning::pattern_metadata — none for an unsupported pattern
#[test]
fn test_pattern_metadata_none_for_unsupported_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .pattern_metadata(PatternMetadataLookupRequest {
            pattern: ReasoningPattern::GraphBased,
        })
        .expect("pattern_metadata should succeed");
    assert!(resp.metadata.is_none());
}

/// @covers: Reasoning::pattern_metadata — carries the requested pattern
#[test]
fn test_pattern_metadata_carries_pattern_edge() {
    let r = reasoner(ReasoningPattern::TreeOfThought);
    let resp = r
        .pattern_metadata(PatternMetadataLookupRequest {
            pattern: ReasoningPattern::TreeOfThought,
        })
        .expect("pattern_metadata should succeed");
    let meta = resp.metadata.expect("metadata");
    assert_eq!(meta.pattern, ReasoningPattern::TreeOfThought);
}

// --- validate_problem ---

/// @covers: Reasoning::validate_problem — accepts a non-empty problem
#[test]
fn test_validate_problem_accepts_nonempty_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let result = r.validate_problem(ProblemValidationRequest { problem: "solve x" });
    assert!(
        matches!(result, Ok(())),
        "non-empty problem should be valid"
    );
}

/// @covers: Reasoning::validate_problem — rejects an empty problem
#[test]
fn test_validate_problem_rejects_empty_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r
        .validate_problem(ProblemValidationRequest { problem: "" })
        .is_err());
}

/// @covers: Reasoning::validate_problem — whitespace-only is rejected
#[test]
fn test_validate_problem_rejects_whitespace_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    assert!(r
        .validate_problem(ProblemValidationRequest { problem: "   \t " })
        .is_err());
}

// --- next_step ---

/// @covers: Reasoning::next_step — produces a step at the next index
#[test]
fn test_next_step_uses_next_index_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "solve x".to_string());
    let resp = r
        .next_step(NextStepRequest { process: &process })
        .expect("next_step should succeed");
    assert_eq!(resp.step.index, 0);
}

/// @covers: Reasoning::next_step — index advances with existing steps
#[test]
fn test_next_step_advances_with_steps_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let reason_resp = block_on(r.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .expect("reasoning should succeed");
    let process = *reason_resp.process;
    let expected_index = process.step_count();
    let resp = r
        .next_step(NextStepRequest { process: &process })
        .expect("next_step should succeed");
    assert_eq!(resp.step.index, expected_index);
}

/// @covers: Reasoning::next_step — references the process problem
#[test]
fn test_next_step_references_problem_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "ship it".to_string());
    let resp = r
        .next_step(NextStepRequest { process: &process })
        .expect("next_step should succeed");
    assert!(resp.step.content.contains("ship it"));
}

// --- evaluate_step ---

/// @covers: Reasoning::evaluate_step — succeeds for a non-empty step
#[test]
fn test_evaluate_step_succeeds_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let process = ThinkingProcess::new("p".to_string(), "solve x".to_string());
    let step_resp = r
        .next_step(NextStepRequest { process: &process })
        .expect("next_step should succeed");
    let eval_resp = r
        .evaluate_step(StepEvaluationRequest {
            step: &step_resp.step,
        })
        .expect("evaluate_step should succeed");
    assert!(eval_resp.result.success);
}

/// @covers: Reasoning::evaluate_step — fails for an empty-content step
#[test]
fn test_evaluate_step_fails_on_empty_error() {
    use edge_llm_reasoning::ReasoningStep;
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let step = ReasoningStep::new(0, String::new(), "analysis".to_string());
    let eval_resp = r
        .evaluate_step(StepEvaluationRequest { step: &step })
        .expect("evaluate_step should succeed");
    assert!(!eval_resp.result.success);
}

/// @covers: Reasoning::evaluate_step — confident step stops continuation
#[test]
fn test_evaluate_step_confident_stops_edge() {
    use edge_llm_reasoning::ReasoningStep;
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let step =
        ReasoningStep::new(0, "done".to_string(), "synthesis".to_string()).with_confidence(0.95);
    let eval_resp = r
        .evaluate_step(StepEvaluationRequest { step: &step })
        .expect("evaluate_step should succeed");
    assert!(!eval_resp.result.should_continue);
}

// --- build_chain ---

/// @covers: Reasoning::build_chain — collects processes into a chain
#[test]
fn test_build_chain_collects_processes_happy() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let reason_resp = block_on(r.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .expect("reasoning should succeed");
    let resp = r
        .build_chain(ChainBuildRequest {
            chain_id: "c1",
            processes: vec![*reason_resp.process],
        })
        .expect("build_chain should succeed");
    assert_eq!(resp.chain.process_count(), 1);
}

/// @covers: Reasoning::build_chain — empty input yields an empty chain
#[test]
fn test_build_chain_empty_input_error() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .build_chain(ChainBuildRequest {
            chain_id: "c1",
            processes: vec![],
        })
        .expect("build_chain should succeed");
    assert_eq!(resp.chain.process_count(), 0);
}

/// @covers: Reasoning::build_chain — preserves the chain id
#[test]
fn test_build_chain_preserves_id_edge() {
    let r = reasoner(ReasoningPattern::ChainOfThought);
    let resp = r
        .build_chain(ChainBuildRequest {
            chain_id: "chain-xyz",
            processes: vec![],
        })
        .expect("build_chain should succeed");
    assert_eq!(resp.chain.id, "chain-xyz");
}
