//! Direct coverage of the `edge-domain-pipeline` dependency, independent of
//! `ConversationLoop`/`DefaultConversationTurnStep` — confirms the crate builds and
//! runs standalone.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
};

type SharedIntLog = Arc<Mutex<Vec<i32>>>;
type BoxedStep = Arc<dyn Step<Ctx = SharedIntLog, ExecutionError = String>>;

struct AppendStep {
    value: i32,
}

#[async_trait]
impl Step for AppendStep {
    type Ctx = SharedIntLog;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, SharedIntLog>) -> Result<(), String> {
        req.ctx.lock().unwrap().push(self.value);
        Ok(())
    }
}

struct FailingStep;

#[async_trait]
impl Step for FailingStep {
    type Ctx = SharedIntLog;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, SharedIntLog>) -> Result<(), String> {
        Err("forced failure".to_string())
    }
}

/// @covers: PipelineSvc::build — happy: sequential steps each mutate shared state in order
#[tokio::test]
async fn test_pipeline_runs_steps_sequentially_happy() {
    let steps: Vec<BoxedStep> = vec![
        Arc::new(AppendStep { value: 1 }),
        Arc::new(AppendStep { value: 2 }),
        Arc::new(AppendStep { value: 3 }),
    ];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });

    let mut ctx: SharedIntLog = Arc::new(Mutex::new(Vec::new()));
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("run ok");

    assert_eq!(*ctx.lock().unwrap(), vec![1, 2, 3]);
}

/// @covers: PipelineSvc::build — error: a failing step surfaces as PipelineError::StepFailed
#[tokio::test]
async fn test_pipeline_failure_surfaces_as_step_failed_error() {
    let steps: Vec<BoxedStep> = vec![Arc::new(AppendStep { value: 1 }), Arc::new(FailingStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });

    let mut ctx: SharedIntLog = Arc::new(Mutex::new(Vec::new()));
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;

    match result {
        Err(PipelineError::StepFailed(step_err)) => {
            assert_eq!(step_err.cause, "forced failure");
        }
        other => panic!("expected StepFailed, got {other:?}"),
    }
    // The first step still ran before the failure halted the pipeline.
    assert_eq!(*ctx.lock().unwrap(), vec![1]);
}
