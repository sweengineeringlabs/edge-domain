//! Direct coverage of the `edge-domain-pipeline` dependency, independent of
//! `ToolCallLoop`/`ToolCallStep` — confirms the crate builds and runs standalone.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_pipeline::{
    ContextMutationRequest, ParallelBranchFailure, ParallelConfig, ParallelStepBuilder,
    ParallelStepError, ParallelStepSvc, Step,
};
use std::sync::{Arc, Mutex};

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

/// @covers: ParallelStepSvc::build — happy: concurrent branches each mutate shared state
#[tokio::test]
async fn test_parallel_step_runs_branches_concurrently_happy() {
    let steps: Vec<BoxedStep> = vec![
        Arc::new(AppendStep { value: 1 }),
        Arc::new(AppendStep { value: 2 }),
        Arc::new(AppendStep { value: 3 }),
    ];
    let parallel_step = ParallelStepSvc::build(ParallelStepBuilder {
        steps,
        config: ParallelConfig::default(),
        event_bus: None,
    });

    let mut ctx: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    parallel_step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("run ok");

    let mut values = ctx.lock().unwrap().clone();
    values.sort();
    assert_eq!(values, vec![1, 2, 3]);
}

/// @covers: ParallelStepSvc::build — error: a failing branch surfaces as ParallelStepError
#[tokio::test]
async fn test_parallel_step_failure_surfaces_as_parallel_step_error_error() {
    let steps: Vec<BoxedStep> = vec![Arc::new(FailingStep)];
    let parallel_step = ParallelStepSvc::build(ParallelStepBuilder {
        steps,
        config: ParallelConfig::default(),
        event_bus: None,
    });

    let mut ctx: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    let result = parallel_step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await;

    match result {
        Err(ParallelStepError { failures }) => {
            assert_eq!(failures.len(), 1);
            assert!(
                matches!(&failures[0], ParallelBranchFailure::Failed(e) if e.cause == "forced failure")
            );
        }
        Ok(()) => panic!("expected ParallelStepError"),
    }
}
