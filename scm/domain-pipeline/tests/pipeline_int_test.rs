//! @covers Pipeline trait
//! Comprehensive trait implementation tests for Pipeline interface.
//! Ensures all trait methods have proper test coverage across happy, error, and edge paths.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineEmptinessRequest, PipelineError, PipelineSvc, Step, StepCountRequest, StepNameRequest,
    StepNameResponse,
};
use std::time::Duration;

struct AlwaysPassStep;

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for AlwaysPassStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "always-pass".to_string(),
        })
    }
}

struct FailureStep {
    reason: String,
}

impl FailureStep {
    fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Step<(), String> for FailureStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, ()>) -> Result<(), String> {
        Err(self.reason.clone())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "failure".to_string(),
        })
    }
}

struct CounterStep {
    value: i32,
}

impl CounterStep {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for CounterStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        *req.ctx += self.value;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "counter".to_string(),
        })
    }
}

// ── execute tests ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_pipeline_execute_empty_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_passing_steps_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep)
            .with(AlwaysPassStep)
            .with(AlwaysPassStep),
    );
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_step_failure_error() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep)
            .with(FailureStep::new("intentional failure"))
            .with(AlwaysPassStep),
    );
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_err());
}

#[tokio::test]
async fn test_pipeline_execute_stops_on_error_error() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep)
            .with(FailureStep::new("stop here"))
            .with(AlwaysPassStep),
    );
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_err());
}

#[tokio::test]
async fn test_pipeline_execute_many_steps_edge() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..500 {
        builder = builder.with(AlwaysPassStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_with_mutations_edge() {
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(CounterStep::new(10))
            .with(CounterStep::new(20))
            .with(CounterStep::new(30)),
    );
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 60);
}

// ── step_count tests ──────────────────────────────────────────────────────────

#[test]
fn test_pipeline_step_count_empty_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[test]
fn test_pipeline_step_count_with_steps_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep)
            .with(AlwaysPassStep)
            .with(AlwaysPassStep),
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        3
    );
}

#[test]
fn test_pipeline_step_count_many_steps_edge() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..250 {
        builder = builder.with(AlwaysPassStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        250
    );
}

#[test]
fn test_pipeline_step_count_consistency_error() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep)
            .with(AlwaysPassStep),
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        2
    );
}

// ── is_empty tests ────────────────────────────────────────────────────────────

#[test]
fn test_pipeline_is_empty_true_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_pipeline_is_empty_false_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(AlwaysPassStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_pipeline_is_empty_many_steps_edge() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..100 {
        builder = builder.with(AlwaysPassStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_pipeline_is_empty_consistency_error() {
    let empty: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    assert!(
        empty
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
    assert_eq!(
        empty
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );

    let non_empty: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(AlwaysPassStep));
    assert!(
        !non_empty
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
    assert_eq!(
        non_empty
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}

// ── config tests ──────────────────────────────────────────────────────────────

#[test]
fn test_pipeline_config_default_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

#[test]
fn test_pipeline_config_custom_happy() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with_timeout(Duration::from_secs(10))
            .emit_lifecycle_events(true)
            .abort_on_error(false),
    );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(10)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

#[test]
fn test_pipeline_config_all_enabled_edge() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with_timeout(Duration::from_secs(5))
            .emit_lifecycle_events(true)
            .abort_on_error(true),
    );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(config.timeout_per_step.is_some());
    assert!(config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

#[test]
fn test_pipeline_config_all_disabled_error() {
    let custom = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config: custom,
        event_bus: None,
    });
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}
