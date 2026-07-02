//! E2E tests for pipeline service (SAF layer).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineEmptinessRequest, PipelineSvc, Step, StepCountRequest,
};
use std::sync::Arc;

#[derive(Clone)]
struct PassStep;

#[async_trait::async_trait]
impl Step for PassStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn test_create_pipeline_happy_returns_pipeline() {
    let steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> = vec![Arc::new(PassStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}

#[test]
fn test_create_pipeline_happy_empty_steps() {
    let steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> = vec![];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_create_pipeline_with_config_happy_applies_config() {
    use std::time::Duration;

    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String> {
        steps: vec![],
        config,
        event_bus: None,
    });
    let result_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(result_config.timeout_per_step, Some(Duration::from_secs(5)));
    assert!(result_config.emit_lifecycle_events);
    assert!(!result_config.abort_on_error);
}

#[test]
fn test_create_pipeline_with_config_happy_default_config() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String> {
        steps: vec![],
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let result_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(result_config.timeout_per_step.is_none());
    assert!(!result_config.emit_lifecycle_events);
    assert!(result_config.abort_on_error);
}

#[tokio::test]
async fn test_create_pipeline_happy_executes() {
    let steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> =
        vec![Arc::new(PassStep), Arc::new(PassStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_create_pipeline_with_config_happy_executes() {
    let steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> = vec![Arc::new(PassStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}
