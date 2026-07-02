//! @covers ParallelExecutor trait — happy/error/edge scenarios for branch_count and new_builder.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, ParallelExecutor, ParallelStepBuilder, ParallelStepError,
    ParallelStepSvc, PipelineError, Step, StepCountRequest, StepCountResponse, StepNameRequest,
    StepNameResponse,
};

struct NoopStep;

#[async_trait::async_trait]
impl Step for NoopStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "noop".to_string(),
        })
    }
}

// ── ParallelExecutor::branch_count ──────────────────────────────────────────────

/// @covers: branch_count
#[test]
fn test_branch_count_happy_counts_added_steps() {
    let executor = ParallelStepSvc::build_executor(
        ParallelStepBuilder::new()
            .with(NoopStep)
            .with(NoopStep)
            .with(NoopStep),
    );
    let count = executor
        .branch_count(StepCountRequest)
        .expect("must succeed")
        .count;
    assert_eq!(count, 3);
}

/// @covers: branch_count
#[test]
fn test_branch_count_error_never_fails_but_never_negative() {
    // branch_count is infallible in practice (Ok(...) always) — assert the invariant
    // a caller actually depends on: the count is never negative (usize) and matches
    // the number of steps added, not some stale/cached value.
    let executor = ParallelStepSvc::build_executor(ParallelStepBuilder::<i32, String>::new());
    let response: StepCountResponse = executor
        .branch_count(StepCountRequest)
        .expect("must succeed");
    assert_eq!(response.count, 0);
}

/// @covers: branch_count
#[test]
fn test_branch_count_edge_empty_fan_out_is_zero() {
    let executor = ParallelStepSvc::build_executor(ParallelStepBuilder::<i32, String>::new());
    assert_eq!(
        executor
            .branch_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

// ── ParallelExecutor::new_builder (Self: Sized default method) ─────────────────

/// A local test-double implementing `ParallelExecutor` directly, purely to reach
/// `new_builder()` — it is a `Self: Sized` default method unreachable through
/// `Box<dyn ParallelExecutor<..>>` (mirrors `NewBuilderProbe` in `tests/pipeline_e2e_test.rs`
/// for `Pipeline::new_builder()`).
struct ParallelExecutorProbe;

#[async_trait::async_trait]
impl Step for ParallelExecutorProbe {
    type Ctx = i32;
    type ExecutionError = ParallelStepError<String>;

    async fn execute(
        &self,
        _req: ContextMutationRequest<'_, i32>,
    ) -> Result<(), ParallelStepError<String>> {
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, PipelineError<ParallelStepError<String>>> {
        Ok(StepNameResponse {
            name: "probe".to_string(),
        })
    }
}

impl ParallelExecutor for ParallelExecutorProbe {
    type BranchError = String;

    fn branch_count(
        &self,
        _req: StepCountRequest,
    ) -> Result<StepCountResponse, PipelineError<String>> {
        Ok(StepCountResponse { count: 0 })
    }
}

/// @covers: new_builder
#[test]
fn test_new_builder_happy_creates_empty_builder() {
    let builder: ParallelStepBuilder<i32, String> = ParallelExecutorProbe::new_builder();
    assert!(builder.steps.is_empty());
}

/// @covers: new_builder
#[test]
fn test_new_builder_error_default_config_has_fail_fast_false() {
    let builder: ParallelStepBuilder<i32, String> = ParallelExecutorProbe::new_builder();
    assert!(
        !builder.config.fail_fast,
        "new_builder's default config must collect all failures, not fail_fast"
    );
}

/// @covers: new_builder
#[test]
fn test_new_builder_edge_usable_to_build_a_real_executor() {
    let builder: ParallelStepBuilder<i32, String> =
        ParallelExecutorProbe::new_builder().with(NoopStep);
    let executor = ParallelStepSvc::build_executor(builder);
    assert_eq!(
        executor
            .branch_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}
