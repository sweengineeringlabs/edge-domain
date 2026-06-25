//! Integration tests for [`PipelineBuilder`] — ADR-048 Phase 1.

use std::sync::Arc;
use std::time::Duration;

use edge_domain_pipeline::{
    PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
};

struct IncrementStep;

#[async_trait::async_trait]
impl Step<i32> for IncrementStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "increment"
    }
}

struct FailStep;

#[async_trait::async_trait]
impl Step<i32> for FailStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("intentional failure".to_string()))
    }

    fn name(&self) -> &str {
        "fail"
    }
}

// ── new_builder (Pipeline trait method) ─────────────────────────────────────

#[test]
fn test_new_builder_happy_creates_empty_builder() {
    let builder: PipelineBuilder<i32> = PipelineBuilder::new();
    assert!(builder.steps.is_empty());
    assert!(builder.config.abort_on_error);
}

#[test]
fn test_new_builder_error_default_config_abort_on_error_true() {
    let builder: PipelineBuilder<i32> = PipelineBuilder::new();
    assert!(builder.config.abort_on_error, "default must abort on error");
    assert!(builder.config.timeout_per_step.is_none(), "default has no timeout");
}

#[test]
fn test_new_builder_edge_two_calls_produce_independent_builders() {
    let b1: PipelineBuilder<i32> = PipelineBuilder::new();
    let b2: PipelineBuilder<i32> = PipelineBuilder::new();
    assert_eq!(b1.steps.len(), b2.steps.len());
    assert_eq!(b1.config.abort_on_error, b2.config.abort_on_error);
}

// ── pipeline_builder ─────────────────────────────────────────────────────────

#[test]
fn test_pipeline_builder_happy_creates_empty() {
    let builder: PipelineBuilder<i32> = PipelineBuilder::new();
    assert!(builder.steps.is_empty());
}

#[test]
fn test_pipeline_builder_error_default_abort_on_error() {
    let builder: PipelineBuilder<i32> = PipelineBuilder::new();
    assert!(builder.config.abort_on_error);
}

#[test]
fn test_pipeline_builder_edge_chained_configuration() {
    let builder: PipelineBuilder<i32> = PipelineBuilder::new()
        .with_timeout(Duration::from_secs(10))
        .abort_on_error(false)
        .emit_lifecycle_events(true);
    assert_eq!(builder.config.timeout_per_step, Some(Duration::from_secs(10)));
    assert!(!builder.config.abort_on_error);
    assert!(builder.config.emit_lifecycle_events);
}

// ── build_pipeline ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_pipeline_build_happy_executes_steps() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .with(IncrementStep),
    );
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("pipeline should succeed");
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_build_happy_empty_pipeline() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32>::new());
    let mut ctx = 0i32;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 0);
}

#[tokio::test]
async fn test_pipeline_build_error_step_failure_propagates() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .with(FailStep),
    );
    let mut ctx = 0i32;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1); // increment ran before fail
}

#[tokio::test]
async fn test_pipeline_build_edge_config_carried_through() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .emit_lifecycle_events(true),
    );
    assert!(pipeline.config().emit_lifecycle_events);
}

// ── with_shared ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_pipeline_with_shared_happy_reuses_step() {
    let step = Arc::new(IncrementStep);
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with_shared(step.clone())
            .with_shared(step),
    );
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_with_shared_error_fail_step_aborts() {
    let step = Arc::new(FailStep);
    let pipeline = PipelineSvc::build(PipelineBuilder::new().with_shared(step));
    let mut ctx = 0i32;
    assert!(pipeline.run(&mut ctx).await.is_err());
}

#[tokio::test]
async fn test_pipeline_with_shared_edge_mix_owned_and_shared() {
    let shared = Arc::new(IncrementStep);
    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(IncrementStep)
            .with_shared(shared),
    );
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 2);
}

// ── PipelineConfig ────────────────────────────────────────────────────────────

#[test]
fn test_pipeline_config_happy_default_values() {
    let config = PipelineConfig::default();
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

#[test]
fn test_pipeline_config_error_abort_on_error_false() {
    let config = PipelineConfig {
        abort_on_error: false,
        ..PipelineConfig::default()
    };
    assert!(!config.abort_on_error);
}

#[test]
fn test_pipeline_config_edge_all_options_set() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_millis(100)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    assert!(config.timeout_per_step.is_some());
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}
