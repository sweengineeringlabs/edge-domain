//! `Reasoning` impl for `LinearReasoning`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
};

use crate::api::Reasoning;
use crate::api::ReasoningError;
use crate::api::{
    ChainBuildRequest, ChainBuildResponse, LinearReasoning, NextStepRequest, NextStepResponse,
    PatternMetadata, PatternMetadataLookupRequest, PatternMetadataLookupResponse, ReasonRequest,
    ReasonResponse, ReasoningChain, ReasoningPattern, ReasoningStep, StepEvaluationRequest,
    StepEvaluationResponse, StepResult, SupportedPatternsRequest, SupportedPatternsResponse,
    ThinkingProcess,
};
use crate::core::reasoning::DefaultReasoningStep;

impl LinearReasoning {
    /// Construct a reasoner bound to the given pattern.
    pub fn new(pattern: ReasoningPattern) -> Self {
        Self { pattern }
    }

    /// Pattern this reasoner executes.
    pub fn pattern(&self) -> ReasoningPattern {
        self.pattern
    }
}

#[async_trait]
impl Reasoning for LinearReasoning {
    async fn reason(&self, req: ReasonRequest<'_>) -> Result<ReasonResponse, ReasoningError> {
        self.validate_problem(crate::api::ProblemValidationRequest {
            problem: req.problem,
        })?;
        if !self
            .supports_pattern(crate::api::PatternSupportRequest {
                pattern: req.pattern,
            })?
            .supported
        {
            return Err(ReasoningError::UnsupportedPattern {
                pattern: req.pattern.as_str().to_string(),
            });
        }
        let mut process =
            ThinkingProcess::new(format!("proc-{}", req.pattern.as_str()), req.problem.into());

        let reasoner: Arc<dyn Reasoning> = Arc::new(self.clone());
        let step: Arc<dyn Step<Ctx = ThinkingProcess, ExecutionError = ReasoningError>> =
            Arc::new(DefaultReasoningStep { reasoner });
        let steps = vec![step; req.pattern.expected_step_count() as usize];

        let pipeline = PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });

        pipeline
            .run(ContextMutationRequest { ctx: &mut process })
            .await
            .map_err(|e| match e {
                PipelineError::StepFailed(step_err) => step_err.cause,
                other => ReasoningError::Unknown(other.to_string()),
            })?;

        Ok(ReasonResponse {
            process: Box::new(process.complete(format!("conclusion for: {}", req.problem))),
        })
    }

    fn supported_patterns(
        &self,
        _req: SupportedPatternsRequest,
    ) -> Result<SupportedPatternsResponse, ReasoningError> {
        Ok(SupportedPatternsResponse {
            patterns: vec![self.pattern],
        })
    }

    fn pattern_metadata(
        &self,
        req: PatternMetadataLookupRequest,
    ) -> Result<PatternMetadataLookupResponse, ReasoningError> {
        let metadata = if self
            .supports_pattern(crate::api::PatternSupportRequest {
                pattern: req.pattern,
            })?
            .supported
        {
            Some(Box::new(PatternMetadata::new(req.pattern)))
        } else {
            None
        };
        Ok(PatternMetadataLookupResponse { metadata })
    }

    fn validate_problem(
        &self,
        req: crate::api::ProblemValidationRequest<'_>,
    ) -> Result<(), ReasoningError> {
        if req.problem.trim().is_empty() {
            return Err(ReasoningError::InvalidState(
                "problem statement must not be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn next_step(&self, req: NextStepRequest<'_>) -> Result<NextStepResponse, ReasoningError> {
        let index = req.process.step_count();
        Ok(NextStepResponse {
            step: Box::new(ReasoningStep::new(
                index,
                format!("next step for: {}", req.process.problem),
                "analysis".to_string(),
            )),
        })
    }

    fn evaluate_step(
        &self,
        req: StepEvaluationRequest<'_>,
    ) -> Result<StepEvaluationResponse, ReasoningError> {
        let result = if req.step.content.trim().is_empty() {
            StepResult::failure("step has no content".to_string())
        } else {
            StepResult::success(req.step.content.clone())
                .with_continuation(!req.step.is_confident())
        };
        Ok(StepEvaluationResponse {
            result: Box::new(result),
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    fn reasoner() -> LinearReasoning {
        LinearReasoning::new(ReasoningPattern::ChainOfThought)
    }

    #[test]
    fn test_new_binds_given_pattern() {
        assert_eq!(reasoner().pattern(), ReasoningPattern::ChainOfThought);
    }

    #[test]
    fn test_reason_produces_completed_process() {
        let response = block_on(reasoner().reason(ReasonRequest {
            problem: "solve x",
            pattern: ReasoningPattern::ChainOfThought,
        }))
        .expect("reasoning should succeed");
        assert!(response.process.is_complete);
        assert_eq!(response.process.step_count(), 3);
    }

    #[test]
    fn test_reason_rejects_unsupported_pattern() {
        let result = block_on(reasoner().reason(ReasonRequest {
            problem: "solve x",
            pattern: ReasoningPattern::GraphBased,
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_problem_rejects_blank() {
        assert!(reasoner()
            .validate_problem(crate::api::ProblemValidationRequest { problem: "   " })
            .is_err());
    }

    #[test]
    fn test_supported_patterns_lists_configured_pattern() {
        assert_eq!(
            reasoner()
                .supported_patterns(SupportedPatternsRequest)
                .expect("ok")
                .patterns,
            vec![ReasoningPattern::ChainOfThought]
        );
    }

    #[test]
    fn test_evaluate_step_fails_on_empty_content() {
        let step = ReasoningStep::new(0, String::new(), "analysis".to_string());
        let result = reasoner()
            .evaluate_step(StepEvaluationRequest { step: &step })
            .expect("ok")
            .result;
        assert!(!result.success);
    }

    /// @covers: pattern
    #[test]
    fn test_pattern_returns_configured_pattern() {
        assert_eq!(reasoner().pattern(), ReasoningPattern::ChainOfThought);
    }

    /// @covers: reason
    #[test]
    fn test_reason_step_count_matches_non_default_pattern_happy() {
        let response = block_on(LinearReasoning::new(ReasoningPattern::FewShot).reason(
            ReasonRequest {
                problem: "solve x",
                pattern: ReasoningPattern::FewShot,
            },
        ))
        .expect("reasoning should succeed");
        assert_eq!(response.process.step_count(), 2);
    }

    /// @covers: reason
    #[test]
    fn test_reason_step_indices_are_sequential_edge() {
        let response = block_on(reasoner().reason(ReasonRequest {
            problem: "solve x",
            pattern: ReasoningPattern::ChainOfThought,
        }))
        .expect("reasoning should succeed");
        for (i, s) in response.process.steps.iter().enumerate() {
            assert_eq!(s.index, i);
        }
    }
}
