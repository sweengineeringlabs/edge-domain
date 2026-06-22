//! Integration test verifying that domain-pipeline is wired into edge-domain.
//!
//! @covers edge-domain pipeline feature integration

#![cfg(feature = "pipeline")]

use edge_domain::{Pipeline, Step, PipelineError, DefaultPipeline, PipelineBuilder, PipelineConfig, NoopStep, AlwaysPassStep};
use std::time::Duration;

#[test]
fn test_pipeline_types_accessible_through_edge_domain() {
    let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
        .with(NoopStep)
        .with(AlwaysPassStep::new())
        .with_timeout(Duration::from_secs(5))
        .build();

    assert_eq!(pipeline.step_count(), 2);
    assert!(!pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_execution_through_edge_domain() {
    let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
        .with(NoopStep)
        .with(AlwaysPassStep::new())
        .build();

    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
}

#[test]
fn test_pipeline_error_through_edge_domain() {
    let err = PipelineError::StepFailed("test".to_string());
    assert_eq!(format!("{}", err).len() > 0, true);
}

#[test]
fn test_pipeline_dyn_dispatch_through_edge_domain() {
    let step: Box<dyn Step<i32>> = Box::new(NoopStep);
    assert_eq!(step.name(), "noop");
}

#[test]
fn test_pipeline_config_through_edge_domain() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };

    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(10)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}
