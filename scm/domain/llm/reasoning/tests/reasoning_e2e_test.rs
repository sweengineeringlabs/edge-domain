//! Layer-level e2e coverage for the `Reasoning` trait via a test-double implementer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_llm_reasoning::*;

struct ReasoningDouble;

#[async_trait]
impl Reasoning for ReasoningDouble {
    async fn reason(&self, req: ReasonRequest<'_>) -> Result<ReasonResponse, ReasoningError> {
        Ok(ReasonResponse {
            process: Box::new(
                ThinkingProcess::new("proc".to_string(), req.problem.to_string())
                    .complete("done".to_string()),
            ),
        })
    }

    fn supported_patterns(
        &self,
        _req: SupportedPatternsRequest,
    ) -> Result<SupportedPatternsResponse, ReasoningError> {
        Ok(SupportedPatternsResponse {
            patterns: vec![ReasoningPattern::ChainOfThought],
        })
    }

    fn pattern_metadata(
        &self,
        req: PatternMetadataLookupRequest,
    ) -> Result<PatternMetadataLookupResponse, ReasoningError> {
        if req.pattern == ReasoningPattern::ChainOfThought {
            Ok(PatternMetadataLookupResponse {
                metadata: Some(Box::new(PatternMetadataBuilder::new(req.pattern).build())),
            })
        } else {
            Ok(PatternMetadataLookupResponse { metadata: None })
        }
    }

    fn validate_problem(&self, req: ProblemValidationRequest<'_>) -> Result<(), ReasoningError> {
        if req.problem.trim().is_empty() {
            return Err(ReasoningError::InvalidState("empty problem".to_string()));
        }
        Ok(())
    }

    fn next_step(&self, req: NextStepRequest<'_>) -> Result<NextStepResponse, ReasoningError> {
        Ok(NextStepResponse {
            step: Box::new(ReasoningStep::new(
                req.process.step_count(),
                "next".to_string(),
                "analysis".to_string(),
            )),
        })
    }

    fn evaluate_step(
        &self,
        req: StepEvaluationRequest<'_>,
    ) -> Result<StepEvaluationResponse, ReasoningError> {
        Ok(StepEvaluationResponse {
            result: Box::new(StepResult::success(req.step.content.clone())),
        })
    }

    fn build_chain(
        &self,
        req: ChainBuildRequest<'_>,
    ) -> Result<ChainBuildResponse, ReasoningError> {
        let mut chain = ReasoningChain::new(req.chain_id.to_string());
        for process in req.processes {
            chain.add_process(process);
        }
        Ok(ChainBuildResponse {
            chain: Box::new(chain),
        })
    }
}

/// @covers: Reasoning::reason — happy path completes a process
#[test]
fn test_reason_completes_process_happy() {
    let response = futures::executor::block_on(ReasoningDouble.reason(ReasonRequest {
        problem: "solve x",
        pattern: ReasoningPattern::ChainOfThought,
    }))
    .expect("reason ok");
    assert!(response.process.is_complete);
}

/// @covers: Reasoning::supported_patterns — reports the double's configured pattern
#[test]
fn test_supported_patterns_reports_configured_pattern_happy() {
    let patterns = ReasoningDouble
        .supported_patterns(SupportedPatternsRequest)
        .expect("ok")
        .patterns;
    assert_eq!(patterns, vec![ReasoningPattern::ChainOfThought]);
}

/// @covers: Reasoning::supports_pattern — default impl checks supported_patterns
#[test]
fn test_supports_pattern_true_for_supported_happy() {
    let supported = ReasoningDouble
        .supports_pattern(PatternSupportRequest {
            pattern: ReasoningPattern::ChainOfThought,
        })
        .expect("ok")
        .supported;
    assert!(supported);
}

/// @covers: Reasoning::pattern_metadata — unsupported pattern returns None
#[test]
fn test_pattern_metadata_none_for_unsupported_error() {
    let metadata = ReasoningDouble
        .pattern_metadata(PatternMetadataLookupRequest {
            pattern: ReasoningPattern::GraphBased,
        })
        .expect("ok")
        .metadata;
    assert!(metadata.is_none());
}

/// @covers: Reasoning::validate_problem — rejects a blank problem
#[test]
fn test_validate_problem_rejects_blank_error() {
    let result = ReasoningDouble.validate_problem(ProblemValidationRequest { problem: "   " });
    assert!(result.is_err());
}

/// @covers: Reasoning::next_step — edge case with an empty process
#[test]
fn test_next_step_from_empty_process_edge() {
    let process = ThinkingProcess::new("p1".to_string(), "q".to_string());
    let step = ReasoningDouble
        .next_step(NextStepRequest { process: &process })
        .expect("ok")
        .step;
    assert_eq!(step.index, 0);
}

/// @covers: Reasoning::evaluate_step — echoes content into a successful result
#[test]
fn test_evaluate_step_echoes_content_happy() {
    let step = ReasoningStep::new(0, "hi".to_string(), "analysis".to_string());
    let result = ReasoningDouble
        .evaluate_step(StepEvaluationRequest { step: &step })
        .expect("ok")
        .result;
    assert_eq!(result.output, "hi");
}

/// @covers: Reasoning::build_chain — edge case with no processes
#[test]
fn test_build_chain_empty_processes_edge() {
    let chain = ReasoningDouble
        .build_chain(ChainBuildRequest {
            chain_id: "c1",
            processes: vec![],
        })
        .expect("ok")
        .chain;
    assert_eq!(chain.process_count(), 0);
}
