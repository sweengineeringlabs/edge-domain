//! `Reasoning` impl for `LinearReasoning`.

use async_trait::async_trait;

use crate::api::Reasoning;
use crate::api::ReasoningError;
use crate::api::{
    LinearReasoning, PatternMetadata, ReasoningChain, ReasoningPattern, ReasoningStep, StepResult,
    ThinkingProcess,
};

#[async_trait]
impl Reasoning for LinearReasoning {
    async fn reason(
        &self,
        problem: &str,
        pattern: ReasoningPattern,
    ) -> Result<ThinkingProcess, ReasoningError> {
        self.validate_problem(problem)?;
        if !self.supports_pattern(pattern) {
            return Err(ReasoningError::UnsupportedPattern {
                pattern: pattern.as_str().to_string(),
            });
        }
        let mut process =
            ThinkingProcess::new(format!("proc-{}", pattern.as_str()), problem.into());
        let count = pattern.expected_step_count();
        for index in 0..count {
            let step = ReasoningStep::new(
                index as usize,
                format!("step {index} reasoning about: {problem}"),
                "analysis".to_string(),
            )
            .with_confidence(0.9)
            .with_tokens(1);
            process.add_step(step);
        }
        Ok(process.complete(format!("conclusion for: {problem}")))
    }

    fn supported_patterns(&self) -> Vec<ReasoningPattern> {
        vec![self.pattern]
    }

    fn pattern_metadata(&self, pattern: ReasoningPattern) -> Option<PatternMetadata> {
        if self.supports_pattern(pattern) {
            Some(PatternMetadata::new(pattern))
        } else {
            None
        }
    }

    fn validate_problem(&self, problem: &str) -> Result<(), ReasoningError> {
        if problem.trim().is_empty() {
            return Err(ReasoningError::InvalidState(
                "problem statement must not be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn next_step(&self, process: &ThinkingProcess) -> ReasoningStep {
        let index = process.step_count();
        ReasoningStep::new(
            index,
            format!("next step for: {}", process.problem),
            "analysis".to_string(),
        )
    }

    fn evaluate_step(&self, step: &ReasoningStep) -> StepResult {
        if step.content.trim().is_empty() {
            StepResult::failure("step has no content".to_string())
        } else {
            StepResult::success(step.content.clone()).with_continuation(!step.is_confident())
        }
    }

    fn build_chain(&self, chain_id: &str, processes: Vec<ThinkingProcess>) -> ReasoningChain {
        let mut chain = ReasoningChain::new(chain_id.to_string());
        for process in processes {
            chain.add_process(process);
        }
        chain
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
    fn test_reason_produces_completed_process() {
        let process = block_on(reasoner().reason("solve x", ReasoningPattern::ChainOfThought))
            .expect("reasoning should succeed");
        assert!(process.is_complete);
        assert_eq!(process.step_count(), 3);
    }

    #[test]
    fn test_reason_rejects_unsupported_pattern() {
        let result = block_on(reasoner().reason("solve x", ReasoningPattern::GraphBased));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_problem_rejects_blank() {
        assert!(reasoner().validate_problem("   ").is_err());
    }

    #[test]
    fn test_supported_patterns_lists_configured_pattern() {
        assert_eq!(
            reasoner().supported_patterns(),
            vec![ReasoningPattern::ChainOfThought]
        );
    }

    #[test]
    fn test_evaluate_step_fails_on_empty_content() {
        let step = ReasoningStep::new(0, String::new(), "analysis".to_string());
        assert!(!reasoner().evaluate_step(&step).success);
    }
}
