//! Integration tests for pipeline service facade.

use edge_domain_pipeline::{PipelineBuilder, PipelineConfig, PipelineSvc, Step, PIPELINE_SVC};
use std::time::Duration;

struct PassStep;

#[async_trait::async_trait]
impl<E: Send + 'static> Step<(), E> for PassStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), E> { Ok(()) }
    fn name(&self) -> &str { "pass" }
}

#[test]
fn test_create_pipeline_empty_happy() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String>::new());
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_create_pipeline_with_steps_happy() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::<(), String>::new().with(PassStep).with(PassStep),
    );
    assert_eq!(pipeline.step_count(), 2);
}

#[test]
fn test_create_pipeline_many_steps_error() {
    let mut builder = PipelineBuilder::<(), String>::new();
    for _ in 0..1000 {
        builder = builder.with(PassStep);
    }
    let pipeline = PipelineSvc::build(builder);
    assert_eq!(pipeline.step_count(), 1000);
}

#[test]
fn test_create_pipeline_empty_edge() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String>::new());
    assert!(pipeline.is_empty());
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_create_pipeline_with_config_timeout_happy() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> { steps: vec![], config, event_bus: None });
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
}

#[test]
fn test_create_pipeline_with_config_lifecycle_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> { steps: vec![], config, event_bus: None });
    assert!(pipeline.config().emit_lifecycle_events);
}

#[test]
fn test_create_pipeline_with_config_all_options_error() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> { steps: vec![], config, event_bus: None });
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(pipeline.config().abort_on_error);
}

#[test]
fn test_create_pipeline_with_config_no_options_edge() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> { steps: vec![], config, event_bus: None });
    assert!(pipeline.config().timeout_per_step.is_none());
    assert!(!pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}

#[test]
fn test_pipeline_svc_constant() {
    assert_eq!(PIPELINE_SVC, "pipeline");
}
