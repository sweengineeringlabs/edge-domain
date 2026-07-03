//! Tests exercising the `edge-domain-pipeline` dependency directly against
//! `ThinkingProcess`/`ReasoningError`, independent of `LinearReasoning`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
    StepNameRequest, StepNameResponse,
};
use edge_llm_reasoning::{ReasoningError, ReasoningStep, ThinkingProcess};
use futures::executor::block_on;

struct AppendStep {
    content: String,
}

#[async_trait]
impl Step for AppendStep {
    type Ctx = ThinkingProcess;
    type ExecutionError = ReasoningError;

    async fn execute(
        &self,
        req: ContextMutationRequest<'_, ThinkingProcess>,
    ) -> Result<(), ReasoningError> {
        let index = req.ctx.step_count();
        req.ctx.add_step(ReasoningStep::new(
            index,
            self.content.clone(),
            "analysis".to_string(),
        ));
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, PipelineError<ReasoningError>> {
        Ok(StepNameResponse {
            name: "append_step".to_string(),
        })
    }
}

struct FailingStep;

#[async_trait]
impl Step for FailingStep {
    type Ctx = ThinkingProcess;
    type ExecutionError = ReasoningError;

    async fn execute(
        &self,
        _req: ContextMutationRequest<'_, ThinkingProcess>,
    ) -> Result<(), ReasoningError> {
        Err(ReasoningError::StepFailed {
            step: 0,
            reason: "forced failure".to_string(),
        })
    }
}

/// @covers: Pipeline::run — mutates the ThinkingProcess context happy
#[test]
fn test_pipeline_run_mutates_thinking_process_context_happy() {
    let steps: Vec<
        std::sync::Arc<dyn Step<Ctx = ThinkingProcess, ExecutionError = ReasoningError>>,
    > = vec![
        std::sync::Arc::new(AppendStep {
            content: "first".to_string(),
        }),
        std::sync::Arc::new(AppendStep {
            content: "second".to_string(),
        }),
    ];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });

    let mut process = ThinkingProcess::new("p1".to_string(), "solve x".to_string());
    block_on(pipeline.run(ContextMutationRequest { ctx: &mut process })).expect("run ok");

    assert_eq!(process.step_count(), 2);
    assert_eq!(process.steps[0].content, "first");
    assert_eq!(process.steps[1].content, "second");
}

/// @covers: Pipeline::run — step failure surfaces as the original ReasoningError error
#[test]
fn test_pipeline_step_failure_surfaces_as_reasoning_error_error() {
    let steps: Vec<
        std::sync::Arc<dyn Step<Ctx = ThinkingProcess, ExecutionError = ReasoningError>>,
    > = vec![std::sync::Arc::new(FailingStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });

    let mut process = ThinkingProcess::new("p1".to_string(), "solve x".to_string());
    let result = block_on(pipeline.run(ContextMutationRequest { ctx: &mut process }));

    match result {
        Err(PipelineError::StepFailed(step_err)) => {
            assert!(matches!(step_err.cause, ReasoningError::StepFailed { .. }));
        }
        other => panic!("expected PipelineError::StepFailed, got {other:?}"),
    }
}
