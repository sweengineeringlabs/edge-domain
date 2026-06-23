//! Comprehensive SAF services test coverage.
//!
//! Tests for PipelineService and ValidatorService with happy, error, and edge scenarios.

use edge_domain_pipeline::{PipelineConfig, Step, PipelineError, PipelineService, ValidatorService, Pipeline};
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Test Doubles
// =============================================================================

#[derive(Clone)]
struct TestPassStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for TestPassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "test-pass"
    }
}

#[derive(Clone)]
struct TestMutatingStep;

#[async_trait::async_trait]
impl Step<i32> for TestMutatingStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "test-mutating"
    }
}

// =============================================================================
// PipelineService::create_pipeline Tests
// =============================================================================

/// @covers: PipelineService::create_pipeline
#[tokio::test]
async fn test_create_pipeline_empty_happy() {
    let pipeline: Box<dyn Pipeline<i32>> = PipelineService::create_pipeline(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

/// @covers: PipelineService::create_pipeline
#[tokio::test]
async fn test_create_pipeline_with_steps_happy() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep), Arc::new(TestPassStep)];
    let pipeline = PipelineService::create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: PipelineService::create_pipeline
#[tokio::test]
async fn test_create_pipeline_executes_steps_happy() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestMutatingStep), Arc::new(TestMutatingStep)];
    let pipeline = PipelineService::create_pipeline(steps);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 2);
}

/// @covers: PipelineService::create_pipeline
#[tokio::test]
async fn test_create_pipeline_many_steps_edge() {
    let steps: Vec<Arc<dyn Step<i32>>> =
        (0..100).map(|_| Arc::new(TestPassStep) as Arc<dyn Step<i32>>).collect();
    let pipeline = PipelineService::create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 100);
}

// =============================================================================
// PipelineService::create_pipeline_with_config Tests
// =============================================================================

/// @covers: PipelineService::create_pipeline_with_config
#[tokio::test]
async fn test_create_pipeline_with_config_default_happy() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep)];
    let config = PipelineConfig::default();
    let pipeline = PipelineService::create_pipeline_with_config(steps, config.clone());
    assert_eq!(pipeline.config().timeout_per_step, None);
    assert!(!pipeline.config().emit_lifecycle_events);
    assert!(pipeline.config().abort_on_error);
}

/// @covers: PipelineService::create_pipeline_with_config
#[tokio::test]
async fn test_create_pipeline_with_config_custom_happy() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep)];
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = PipelineService::create_pipeline_with_config(steps, config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}

/// @covers: PipelineService::create_pipeline_with_config
#[tokio::test]
async fn test_create_pipeline_with_config_executes_happy() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestMutatingStep)];
    let config = PipelineConfig::default();
    let pipeline = PipelineService::create_pipeline_with_config(steps, config);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

/// @covers: PipelineService::create_pipeline_with_config
#[tokio::test]
async fn test_create_pipeline_with_config_respects_config_edge() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![];
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(30)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline = PipelineService::create_pipeline_with_config(steps, config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(30)));
    assert!(pipeline.config().emit_lifecycle_events);
}

// =============================================================================
// ValidatorService::create_validator Tests
// =============================================================================

/// @covers: ValidatorService::create_validator
#[test]
fn test_create_validator_enabled_happy() {
    let validator = ValidatorService::create_validator(true);
    assert!(validator.is_enabled());
}

/// @covers: ValidatorService::create_validator
#[test]
fn test_create_validator_disabled_happy() {
    let validator = ValidatorService::create_validator(false);
    assert!(!validator.is_enabled());
}

/// @covers: ValidatorService::create_validator
#[test]
fn test_create_validator_instance_independent_edge() {
    let validator1 = ValidatorService::create_validator(true);
    let validator2 = ValidatorService::create_validator(false);
    assert!(validator1.is_enabled());
    assert!(!validator2.is_enabled());
}

/// @covers: ValidatorService::create_validator
#[test]
fn test_create_validator_multiple_instances_edge() {
    let v1 = ValidatorService::create_validator(true);
    let v2 = ValidatorService::create_validator(true);
    let v3 = ValidatorService::create_validator(false);
    assert!(v1.is_enabled());
    assert!(v2.is_enabled());
    assert!(!v3.is_enabled());
}
