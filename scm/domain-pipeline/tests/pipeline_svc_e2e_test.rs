//! E2E tests for pipeline service (SAF layer).

use edge_domain_pipeline::{PipelineConfig, Step, PipelineError, create_pipeline, create_pipeline_with_config};
use std::sync::Arc;

#[derive(Clone)]
struct PassStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for PassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "pass"
    }
}

#[test]
fn test_create_pipeline_happy_returns_pipeline() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(PassStep)];
    let pipeline = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 1);
}

#[test]
fn test_create_pipeline_happy_empty_steps() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![];
    let pipeline = create_pipeline(steps);
    assert!(pipeline.is_empty());
}

#[test]
fn test_create_pipeline_with_config_happy_applies_config() {
    use std::time::Duration;

    let steps: Vec<Arc<dyn Step<i32>>> = vec![];
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = create_pipeline_with_config(steps, config);
    let result_config = pipeline.config();
    assert_eq!(result_config.timeout_per_step, Some(Duration::from_secs(5)));
    assert!(result_config.emit_lifecycle_events);
    assert!(!result_config.abort_on_error);
}

#[test]
fn test_create_pipeline_with_config_happy_default_config() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![];
    let config = PipelineConfig::default();
    let pipeline = create_pipeline_with_config(steps, config);
    let result_config = pipeline.config();
    assert!(result_config.timeout_per_step.is_none());
    assert!(!result_config.emit_lifecycle_events);
    assert!(result_config.abort_on_error);
}

#[tokio::test]
async fn test_create_pipeline_happy_executes() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(PassStep), Arc::new(PassStep)];
    let pipeline = create_pipeline(steps);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_create_pipeline_with_config_happy_executes() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(PassStep)];
    let config = PipelineConfig::default();
    let pipeline = create_pipeline_with_config(steps, config);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}
