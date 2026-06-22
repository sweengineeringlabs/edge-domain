//! Integration tests for pipeline service facade.

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineError, PIPELINE_SVC};
use std::sync::Arc;
use std::time::Duration;

struct PassStep;

#[async_trait::async_trait]
impl Step<()> for PassStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "pass"
    }
}

// Test create_pipeline factory
/// @covers: create_pipeline
#[test]
fn test_pipeline_svc_create_pipeline_happy_empty() {
    let pipeline: _ = create_pipeline::<()>(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

/// @covers: create_pipeline
#[test]
fn test_pipeline_svc_create_pipeline_happy_with_steps() {
    let pipeline = create_pipeline(vec![
        Arc::new(PassStep),
        Arc::new(PassStep),
    ]);
    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: create_pipeline
#[test]
fn test_pipeline_svc_create_pipeline_edge_many_steps() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..100 {
        steps.push(Arc::new(PassStep));
    }
    let pipeline = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 100);
}

// Test create_pipeline_with_config factory
/// @covers: create_pipeline_with_config
#[test]
fn test_pipeline_svc_create_pipeline_with_config_happy_timeout() {
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
}

/// @covers: create_pipeline_with_config
#[test]
fn test_pipeline_svc_create_pipeline_with_config_happy_lifecycle() {
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert!(pipeline.config().emit_lifecycle_events);
}

/// @covers: create_pipeline_with_config
#[test]
fn test_pipeline_svc_create_pipeline_with_config_edge_all_options() {
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(pipeline.config().abort_on_error);
}

// Test PIPELINE_SVC constant
/// @covers: general
#[test]
fn test_pipeline_svc_constant() {
    assert_eq!(PIPELINE_SVC, "pipeline");
}
