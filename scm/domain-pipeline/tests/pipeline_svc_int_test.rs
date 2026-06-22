//! Integration tests for pipeline service facade.

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, Step, PipelineError, PIPELINE_SVC};
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
#[test]
fn test_create_pipeline_empty_happy() {
    let pipeline: _ = create_pipeline::<()>(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_create_pipeline_with_steps_happy() {
    let pipeline = create_pipeline(vec![
        Arc::new(PassStep),
        Arc::new(PassStep),
    ]);
    assert_eq!(pipeline.step_count(), 2);
}

#[test]
fn test_create_pipeline_many_steps_error() {
    // Test that pipeline handles a large number of steps without panicking
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..1000 {
        steps.push(Arc::new(PassStep));
    }
    let pipeline = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 1000);
}

#[test]
fn test_create_pipeline_empty_edge() {
    // Edge case: verify that empty pipeline returns correct step count
    let pipeline: _ = create_pipeline::<()>(vec![]);
    assert!(pipeline.is_empty());
    assert_eq!(pipeline.step_count(), 0);
}

// Test create_pipeline_with_config factory
#[test]
fn test_create_pipeline_with_config_timeout_happy() {
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
}

#[test]
fn test_create_pipeline_with_config_lifecycle_happy() {
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert!(pipeline.config().emit_lifecycle_events);
}

#[test]
fn test_create_pipeline_with_config_all_options_error() {
    // Error scenario: verify config with all flags set
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    // Verify all configurations are preserved
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(pipeline.config().abort_on_error);
}

#[test]
fn test_create_pipeline_with_config_no_options_edge() {
    // Edge case: config with all options disabled
    let config = edge_domain_pipeline::PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline: _ = create_pipeline_with_config::<()>(vec![], config);
    assert!(pipeline.config().timeout_per_step.is_none());
    assert!(!pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}

// Test PIPELINE_SVC constant
#[test]
fn test_pipeline_svc_constant() {
    assert_eq!(PIPELINE_SVC, "pipeline");
}
