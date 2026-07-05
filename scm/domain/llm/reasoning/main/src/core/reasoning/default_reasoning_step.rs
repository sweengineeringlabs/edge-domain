//! `DefaultReasoningStep` — bridges `Reasoning::next_step`/`evaluate_step` into
//! `edge-domain-pipeline`'s `Step` contract.

use std::sync::Arc;

use async_trait::async_trait;
use edge_pipeline::{ContextMutationRequest, Step};

use crate::api::{
    NextStepRequest, Reasoning, ReasoningError, ReasoningStep, StepEvaluationRequest, StepResult,
    ThinkingProcess,
};

pub(crate) struct DefaultReasoningStep {
    pub(crate) reasoner: Arc<dyn Reasoning>,
}

impl DefaultReasoningStep {
    /// Turn a proposed step and its evaluation into either the step to append
    /// or the [`ReasoningError`] to fail the pipeline step with.
    fn apply_evaluation(
        next: ReasoningStep,
        evaluation: StepResult,
    ) -> Result<ReasoningStep, ReasoningError> {
        if !evaluation.success {
            return Err(ReasoningError::StepFailed {
                step: next.index,
                reason: evaluation
                    .error
                    .unwrap_or_else(|| "step evaluation failed".to_string()),
            });
        }
        Ok(next)
    }
}

#[async_trait]
impl Step for DefaultReasoningStep {
    type Ctx = ThinkingProcess;
    type ExecutionError = ReasoningError;

    async fn execute(
        &self,
        req: ContextMutationRequest<'_, ThinkingProcess>,
    ) -> Result<(), ReasoningError> {
        let next = self
            .reasoner
            .next_step(NextStepRequest { process: req.ctx })?
            .step;
        let evaluation = self
            .reasoner
            .evaluate_step(StepEvaluationRequest { step: &next })?
            .result;
        let next = Self::apply_evaluation(*next, *evaluation)?;
        req.ctx.add_step(next);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{LinearReasoning, ReasoningPattern};
    use futures::executor::block_on;

    fn step() -> DefaultReasoningStep {
        DefaultReasoningStep {
            reasoner: Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought)),
        }
    }

    /// @covers: execute
    #[test]
    fn test_execute_appends_step_via_next_step_and_evaluate_step_happy() {
        let s = step();
        let mut process = ThinkingProcess::new("p1".to_string(), "solve x".to_string());
        block_on(s.execute(ContextMutationRequest { ctx: &mut process })).expect("ok");
        assert_eq!(process.step_count(), 1);
    }

    /// @covers: execute
    #[test]
    fn test_execute_on_empty_process_uses_index_zero_edge() {
        let s = step();
        let mut process = ThinkingProcess::new("p1".to_string(), "solve x".to_string());
        block_on(s.execute(ContextMutationRequest { ctx: &mut process })).expect("ok");
        assert_eq!(process.steps[0].index, 0);
    }

    /// @covers: apply_evaluation
    #[test]
    fn test_apply_evaluation_passes_through_successful_evaluation_happy() {
        let next = ReasoningStep::new(2, "content".to_string(), "analysis".to_string());
        let evaluation = StepResult {
            success: true,
            output: "ok".to_string(),
            error: None,
            duration_ms: 0,
            should_continue: true,
            next_action: None,
        };
        let result = DefaultReasoningStep::apply_evaluation(next, evaluation).expect("ok");
        assert_eq!(result.index, 2);
    }

    /// @covers: apply_evaluation
    #[test]
    fn test_apply_evaluation_returns_step_failed_when_evaluation_unsuccessful_error() {
        let next = ReasoningStep::new(3, "content".to_string(), "analysis".to_string());
        let evaluation = StepResult {
            success: false,
            output: String::new(),
            error: Some("bad step".to_string()),
            duration_ms: 0,
            should_continue: false,
            next_action: None,
        };
        let result = DefaultReasoningStep::apply_evaluation(next, evaluation);
        assert!(matches!(
            result,
            Err(ReasoningError::StepFailed { step: 3, .. })
        ));
    }
}
