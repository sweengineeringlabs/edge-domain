//! Layer-level coverage for small api/ Request/Response marker types that don't
//! warrant their own dedicated test file — see SEA §5 Option C.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::*;

#[test]
fn test_chain_build_request_response_construct() {
    let req = ChainBuildRequest {
        chain_id: "c1",
        processes: vec![],
    };
    assert_eq!(req.chain_id, "c1");
    let resp = ChainBuildResponse {
        chain: Box::new(ReasoningChain::new("c1".to_string())),
    };
    assert_eq!(resp.chain.id, "c1");
}

#[test]
fn test_next_step_request_response_construct() {
    let process = ThinkingProcess::new("p1".to_string(), "q".to_string());
    let req = NextStepRequest { process: &process };
    assert_eq!(req.process.id, "p1");
    let resp = NextStepResponse {
        step: Box::new(ReasoningStep::new(
            0,
            "x".to_string(),
            "analysis".to_string(),
        )),
    };
    assert_eq!(resp.step.index, 0);
}

#[test]
fn test_pattern_metadata_lookup_request_response_construct() {
    let req = PatternMetadataLookupRequest {
        pattern: ReasoningPattern::ChainOfThought,
    };
    assert_eq!(req.pattern, ReasoningPattern::ChainOfThought);
    let resp = PatternMetadataLookupResponse { metadata: None };
    assert!(resp.metadata.is_none());
}

#[test]
fn test_pattern_support_request_response_construct() {
    let req = PatternSupportRequest {
        pattern: ReasoningPattern::Reflection,
    };
    assert_eq!(req.pattern, ReasoningPattern::Reflection);
    let resp = PatternSupportResponse { supported: true };
    assert!(resp.supported);
}

#[test]
fn test_problem_validation_request_constructs() {
    let req = ProblemValidationRequest { problem: "solve x" };
    assert_eq!(req.problem, "solve x");
}

#[test]
fn test_reasoning_bootstrap_name_request_response_construct() {
    let _req = ReasoningBootstrapNameRequest;
    let resp = ReasoningBootstrapNameResponse {
        name: "reasoning".to_string(),
    };
    assert_eq!(resp.name, "reasoning");
}

#[test]
fn test_reason_request_response_construct() {
    let req = ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    };
    assert_eq!(req.problem, "solve x");
    let resp = ReasonResponse {
        process: Box::new(ThinkingProcess::new(
            "p1".to_string(),
            "solve x".to_string(),
        )),
    };
    assert_eq!(resp.process.problem, "solve x");
}

#[test]
fn test_step_evaluation_request_response_construct() {
    let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
    let req = StepEvaluationRequest { step: &step };
    assert_eq!(req.step.index, 0);
    let resp = StepEvaluationResponse {
        result: Box::new(StepResult::success("ok".to_string())),
    };
    assert!(resp.result.success);
}

#[test]
fn test_supported_patterns_request_response_construct() {
    let _req = SupportedPatternsRequest;
    let resp = SupportedPatternsResponse {
        patterns: vec![ReasoningPattern::ChainOfThought],
    };
    assert_eq!(resp.patterns, vec![ReasoningPattern::ChainOfThought]);
}
