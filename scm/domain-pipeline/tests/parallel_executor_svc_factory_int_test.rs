//! Integration tests — `ParallelStepSvc` construction surface.
//! @covers ParallelStepSvc::build, ParallelStepSvc::build_shared
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use edge_domain_pipeline::{
    ContextMutationRequest, ParallelBranchFailure, ParallelStepBuilder, ParallelStepSvc, Step,
    StepNameRequest, StepNameResponse, PARALLEL_STEP_SVC,
};

struct PassStep(&'static str);

#[async_trait::async_trait]
impl Step for PassStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: self.0.to_string(),
        })
    }
}

struct FailStep(&'static str);

#[async_trait::async_trait]
impl Step for FailStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Err(format!("{} failed", self.0))
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: self.0.to_string(),
        })
    }
}

struct SlowStep;

#[async_trait::async_trait]
impl Step for SlowStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: "slow".to_string(),
        })
    }
}

// ── PARALLEL_STEP_SVC constant ──────────────────────────────────────────────────

/// @covers: general
#[test]
fn test_parallel_step_svc_constant() {
    assert_eq!(PARALLEL_STEP_SVC, "parallel_step");
}

// ── ParallelStepSvc::build ──────────────────────────────────────────────────────

/// @covers: build
#[tokio::test]
async fn test_build_all_branches_succeed_happy() {
    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(PassStep("a"))
            .with(PassStep("b"))
            .with(PassStep("c")),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_ok(), "all-passing fan-out must succeed");
}

/// @covers: build
#[tokio::test]
async fn test_build_some_branches_fail_error() {
    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(PassStep("a"))
            .with(FailStep("b"))
            .with(FailStep("c")),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    let err = result.expect_err("mixed pass/fail fan-out must fail");
    assert_eq!(
        err.failures.len(),
        2,
        "fail_fast defaults to false — both failures must be collected, not just the first"
    );
    let names: Vec<&str> = err
        .failures
        .iter()
        .map(|f| match f {
            ParallelBranchFailure::Failed(e) => e.step_name.as_str(),
            ParallelBranchFailure::TimedOut { step_name } => step_name.as_str(),
            ParallelBranchFailure::Panicked => "<panicked>",
        })
        .collect();
    assert!(names.contains(&"b"));
    assert!(names.contains(&"c"));
}

/// @covers: build
#[tokio::test]
async fn test_build_empty_steps_succeeds_edge() {
    let step = ParallelStepSvc::build(ParallelStepBuilder::<i32, String>::new());
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_ok(), "an empty fan-out has nothing to fail");
}

// ── ParallelStepSvc::build_shared ───────────────────────────────────────────────

/// @covers: build_shared
#[tokio::test]
async fn test_build_shared_all_branches_succeed_happy() {
    let step: Arc<
        dyn Step<Ctx = i32, ExecutionError = edge_domain_pipeline::ParallelStepError<String>>,
    > = ParallelStepSvc::build_shared(
        ParallelStepBuilder::new()
            .with(PassStep("a"))
            .with(PassStep("b")),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_ok());
}

/// @covers: build_shared
#[tokio::test]
async fn test_build_shared_branch_fails_error() {
    let step = ParallelStepSvc::build_shared(
        ParallelStepBuilder::new()
            .with(PassStep("a"))
            .with(FailStep("b")),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
}

/// @covers: build_shared
#[test]
fn test_build_shared_clone_increments_strong_count_edge() {
    let step = ParallelStepSvc::build_shared(
        ParallelStepBuilder::<i32, String>::new().with(PassStep("a")),
    );
    let before = Arc::strong_count(&step);
    let cloned = Arc::clone(&step);
    assert_eq!(Arc::strong_count(&cloned), before + 1);
}

// ── fail_fast cancellation ──────────────────────────────────────────────────────

/// @covers: build
#[tokio::test]
async fn test_build_fail_fast_stops_at_first_failure_edge() {
    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(FailStep("a"))
            .with(SlowStep)
            .fail_fast(true),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    let err = result.expect_err("fail_fast fan-out with one failing branch must fail");
    assert_eq!(
        err.failures.len(),
        1,
        "fail_fast must report exactly the one failure that triggered cancellation"
    );
}

// ── timeout_per_branch ───────────────────────────────────────────────────────────

/// @covers: build
#[tokio::test]
async fn test_build_timeout_per_branch_reports_timed_out_error() {
    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(SlowStep)
            .timeout_per_branch(Duration::from_millis(20)),
    );
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    let err = result.expect_err("branch exceeding its timeout must fail");
    assert!(matches!(
        err.failures.as_slice(),
        [ParallelBranchFailure::TimedOut { .. }]
    ));
}
