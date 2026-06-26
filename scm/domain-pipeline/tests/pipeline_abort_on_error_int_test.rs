//! Integration tests for `abort_on_error` config — ADR-048 Phase 1.

use edge_domain_pipeline::{
    PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
};

struct IncrementStep;

#[async_trait::async_trait]
impl Step<Vec<i32>> for IncrementStep {
    async fn execute(&self, ctx: &mut Vec<i32>) -> Result<(), PipelineError> {
        ctx.push(ctx.len() as i32);
        Ok(())
    }

    fn name(&self) -> &str {
        "increment"
    }
}

struct FailStep;

#[async_trait::async_trait]
impl Step<Vec<i32>> for FailStep {
    async fn execute(&self, _ctx: &mut Vec<i32>) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("fail".to_string()))
    }

    fn name(&self) -> &str {
        "fail"
    }
}

// ── abort_on_error = true (default) ──────────────────────────────────────────

#[tokio::test]
async fn test_abort_on_error_happy_stops_at_first_failure() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .with(FailStep)
            .with(IncrementStep), // should NOT run
    );
    let mut ctx: Vec<i32> = vec![];
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx.len(), 1, "only one step ran before abort");
}

#[tokio::test]
async fn test_abort_on_error_happy_propagates_error() {
    let pipeline = PipelineSvc::build(PipelineBuilder::new().with(FailStep));
    let mut ctx: Vec<i32> = vec![];
    match pipeline.run(&mut ctx).await {
        Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "fail"),
        other => panic!("expected StepFailed, got {:?}", other),
    }
}

#[tokio::test]
async fn test_abort_on_error_edge_succeeds_with_no_errors() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .with(IncrementStep),
    );
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx.len(), 2);
}

// ── abort_on_error = false ────────────────────────────────────────────────────

#[tokio::test]
async fn test_abort_on_error_false_happy_continues_past_failure() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .abort_on_error(false)
            .with(IncrementStep)
            .with(FailStep)
            .with(IncrementStep), // MUST run despite FailStep
    );
    let mut ctx: Vec<i32> = vec![];
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_ok(), "pipeline should succeed when abort_on_error=false");
    assert_eq!(ctx.len(), 2, "both increment steps must have run");
}

#[tokio::test]
async fn test_abort_on_error_false_error_all_steps_run_despite_failures() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .abort_on_error(false)
            .with(FailStep)
            .with(IncrementStep)
            .with(FailStep)
            .with(IncrementStep),
    );
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx.len(), 2, "both increment steps must have run past failures");
}

#[tokio::test]
async fn test_abort_on_error_false_edge_no_steps_returns_ok() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<Vec<i32>>::new().abort_on_error(false));
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_ok());
}

// ── via create_pipeline_with_config ──────────────────────────────────────────

#[tokio::test]
async fn test_abort_on_error_via_config_happy_stops_on_error() {
    use std::sync::Arc;
    let steps: Vec<Arc<dyn Step<Vec<i32>>>> =
        vec![Arc::new(IncrementStep), Arc::new(FailStep), Arc::new(IncrementStep)];
    let config = PipelineConfig { abort_on_error: true, ..PipelineConfig::default() };
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config, event_bus: None });
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_err());
    assert_eq!(ctx.len(), 1);
}

#[tokio::test]
async fn test_abort_on_error_via_config_error_continues_when_false() {
    use std::sync::Arc;
    let steps: Vec<Arc<dyn Step<Vec<i32>>>> =
        vec![Arc::new(IncrementStep), Arc::new(FailStep), Arc::new(IncrementStep)];
    let config = PipelineConfig { abort_on_error: false, ..PipelineConfig::default() };
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config, event_bus: None });
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx.len(), 2);
}

#[tokio::test]
async fn test_abort_on_error_via_config_edge_all_pass() {
    use std::sync::Arc;
    let steps: Vec<Arc<dyn Step<Vec<i32>>>> =
        vec![Arc::new(IncrementStep), Arc::new(IncrementStep)];
    let config = PipelineConfig::default();
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config, event_bus: None });
    let mut ctx: Vec<i32> = vec![];
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx.len(), 2);
}
