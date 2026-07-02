//! Integration tests for pipeline service facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineEmptinessRequest, PipelineSvc, Step, StepCountRequest, PIPELINE_SVC,
};
use std::time::Duration;

struct PassStep;

#[async_trait::async_trait]
impl<E: Send + 'static> Step<(), E> for PassStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, ()>) -> Result<(), E> {
        Ok(())
    }
}

#[test]
fn test_create_pipeline_empty_happy() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String>::new());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[test]
fn test_create_pipeline_with_steps_happy() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::<(), String>::new()
            .with(PassStep)
            .with(PassStep),
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        2
    );
}

#[test]
fn test_create_pipeline_many_steps_error() {
    let mut builder = PipelineBuilder::<(), String>::new();
    for _ in 0..1000 {
        builder = builder.with(PassStep);
    }
    let pipeline = PipelineSvc::build(builder);
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1000
    );
}

#[test]
fn test_create_pipeline_empty_edge() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String>::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[test]
fn test_create_pipeline_with_config_timeout_happy() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert_eq!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .timeout_per_step,
        Some(Duration::from_secs(5))
    );
}

#[test]
fn test_create_pipeline_with_config_lifecycle_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .emit_lifecycle_events
    );
}

#[test]
fn test_create_pipeline_with_config_all_options_error() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> {
        steps: vec![],
        config,
        event_bus: None,
    });
    let result_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(
        result_config.timeout_per_step,
        Some(Duration::from_secs(10))
    );
    assert!(result_config.emit_lifecycle_events);
    assert!(result_config.abort_on_error);
}

#[test]
fn test_create_pipeline_with_config_no_options_edge() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder::<(), String> {
        steps: vec![],
        config,
        event_bus: None,
    });
    let result_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(result_config.timeout_per_step.is_none());
    assert!(!result_config.emit_lifecycle_events);
    assert!(!result_config.abort_on_error);
}

#[test]
fn test_pipeline_svc_constant() {
    assert_eq!(PIPELINE_SVC, "pipeline");
}
