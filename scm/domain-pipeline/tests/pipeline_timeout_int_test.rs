//! Integration tests for `timeout_per_step` / `StepTimeout` — ADR-048 Phase 1.

use std::time::Duration;

use edge_domain_pipeline::{
    PipelineBuilder, PipelineError, PipelineSvc, Step,
};

struct FastStep;

#[async_trait::async_trait]
impl Step<i32> for FastStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "fast"
    }
}

struct SlowStep {
    delay_ms: u64,
}

#[async_trait::async_trait]
impl Step<i32> for SlowStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "slow"
    }
}

// ── timeout enforced → StepTimeout ───────────────────────────────────────────

#[tokio::test]
async fn test_timeout_happy_step_within_limit_succeeds() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(FastStep)
            .with_timeout(Duration::from_secs(5)),
    );
    let mut ctx = 0i32;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_timeout_error_step_exceeds_limit_produces_step_timeout() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(SlowStep { delay_ms: 200 })
            .with_timeout(Duration::from_millis(50)),
    );
    let mut ctx = 0i32;
    match pipeline.run(&mut ctx).await {
        Err(PipelineError::StepTimeout) => {}
        other => panic!("expected StepTimeout, got {:?}", other),
    }
}

#[tokio::test]
async fn test_timeout_edge_no_timeout_set_ignores_slow_step() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(SlowStep { delay_ms: 50 }),
    );
    let mut ctx = 0i32;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

// ── timeout + abort_on_error interaction ─────────────────────────────────────

#[tokio::test]
async fn test_timeout_abort_on_error_true_stops_on_timeout() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .abort_on_error(true)
            .with(SlowStep { delay_ms: 200 })
            .with(FastStep) // should NOT run
            .with_timeout(Duration::from_millis(50)),
    );
    let mut ctx = 0i32;
    let result = pipeline.run(&mut ctx).await;
    assert!(matches!(result, Err(PipelineError::StepTimeout)));
    assert_eq!(ctx, 0, "fast step must not run after timeout abort");
}

#[tokio::test]
async fn test_timeout_abort_on_error_false_continues_after_timeout() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .abort_on_error(false)
            .with(SlowStep { delay_ms: 200 })
            .with(FastStep) // MUST run even after timeout
            .with_timeout(Duration::from_millis(50)),
    );
    let mut ctx = 0i32;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_ok(), "abort_on_error=false must continue past timeout");
    assert_eq!(ctx, 1, "fast step must run after timed-out slow step");
}

#[tokio::test]
async fn test_timeout_edge_all_steps_fast_none_timeout() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(FastStep)
            .with(FastStep)
            .with(FastStep)
            .with_timeout(Duration::from_secs(5)),
    );
    let mut ctx = 0i32;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 3);
}
