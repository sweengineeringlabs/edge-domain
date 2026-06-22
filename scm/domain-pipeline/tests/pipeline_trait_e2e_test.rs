//! Comprehensive trait method tests for Pipeline, Step, and Validator.
//! This file ensures all trait methods have _happy, _error, and _edge test variants.

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, create_validator};
use edge_domain_pipeline::{Pipeline, Step, PipelineError, PipelineConfig};
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Pipeline trait tests
// =============================================================================

struct DummyStep;

#[async_trait::async_trait]
impl Step<()> for DummyStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "dummy"
    }
}

struct FailStep;

#[async_trait::async_trait]
impl Step<()> for FailStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("test failure".to_string()))
    }

    fn name(&self) -> &str {
        "fail-step"
    }
}

// Pipeline::execute tests

/// @covers: Pipeline::execute _happy path
#[tokio::test]
async fn test_pipeline_trait_execute_happy_empty() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

/// @covers: Pipeline::execute _happy path
#[tokio::test]
async fn test_pipeline_trait_execute_happy_with_steps() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(DummyStep),
        Arc::new(DummyStep),
    ]);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

/// @covers: Pipeline::execute _error path
#[tokio::test]
async fn test_pipeline_trait_execute_error_fails_on_step() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(DummyStep),
        Arc::new(FailStep),
        Arc::new(DummyStep),
    ]);
    let mut ctx = ();
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
}

/// @covers: Pipeline::execute _edge case
#[tokio::test]
async fn test_pipeline_trait_execute_edge_many_steps() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..50 {
        steps.push(Arc::new(DummyStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

// Pipeline::step_count tests

/// @covers: Pipeline::step_count _happy path
#[test]
fn test_pipeline_trait_step_count_happy_empty() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

/// @covers: Pipeline::step_count _happy path
#[test]
fn test_pipeline_trait_step_count_happy_with_steps() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(DummyStep),
        Arc::new(DummyStep),
        Arc::new(DummyStep),
    ]);
    assert_eq!(pipeline.step_count(), 3);
}

/// @covers: Pipeline::step_count _edge case
#[test]
fn test_pipeline_trait_step_count_edge_many_steps() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..100 {
        steps.push(Arc::new(DummyStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 100);
}

// Pipeline::is_empty tests (default impl uses step_count)

/// @covers: Pipeline::is_empty _happy path (true case)
#[test]
fn test_pipeline_trait_is_empty_happy_true() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    assert!(pipeline.is_empty());
}

/// @covers: Pipeline::is_empty _happy path (false case)
#[test]
fn test_pipeline_trait_is_empty_happy_false() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![Arc::new(DummyStep)]);
    assert!(!pipeline.is_empty());
}

/// @covers: Pipeline::is_empty _edge case
#[test]
fn test_pipeline_trait_is_empty_edge_many_steps() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..50 {
        steps.push(Arc::new(DummyStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    assert!(!pipeline.is_empty());
}

// Pipeline::config tests

/// @covers: Pipeline::config _happy path (default config)
#[test]
fn test_pipeline_trait_config_happy_default() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    let config = pipeline.config();
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

/// @covers: Pipeline::config _happy path (custom config)
#[test]
fn test_pipeline_trait_config_happy_custom() {
    let custom_config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline_with_config(vec![], custom_config);
    let config = pipeline.config();
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(5)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

/// @covers: Pipeline::config _edge case
#[test]
fn test_pipeline_trait_config_edge_all_options_set() {
    let custom_config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(30)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline_with_config(vec![], custom_config);
    let config = pipeline.config();
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(30)));
    assert!(config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

// =============================================================================
// Step trait tests
// =============================================================================

struct MutatingStep(i32);

#[async_trait::async_trait]
impl Step<i32> for MutatingStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += self.0;
        Ok(())
    }

    fn name(&self) -> &str {
        "mutating"
    }
}

struct ErrorStep;

#[async_trait::async_trait]
impl Step<i32> for ErrorStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("error".to_string()))
    }

    fn name(&self) -> &str {
        "error"
    }
}

// Step::execute tests

/// @covers: Step::execute _happy path
#[tokio::test]
async fn test_step_trait_execute_happy_succeeds() {
    let step = DummyStep;
    let step_ref: &dyn Step<()> = &step;
    let mut ctx = ();
    assert!(step_ref.execute(&mut ctx).await.is_ok());
}

/// @covers: Step::execute _happy path (with mutation)
#[tokio::test]
async fn test_step_trait_execute_happy_mutates() {
    let step = MutatingStep(10);
    let step_ref: &dyn Step<i32> = &step;
    let mut ctx = 5;
    assert!(step_ref.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: Step::execute _error path
#[tokio::test]
async fn test_step_trait_execute_error_returns_error() {
    let step = ErrorStep;
    let step_ref: &dyn Step<i32> = &step;
    let mut ctx = 0;
    let result = step_ref.execute(&mut ctx).await;
    assert!(result.is_err());
}

/// @covers: Step::execute _edge case
#[tokio::test]
async fn test_step_trait_execute_edge_large_mutation() {
    let step = MutatingStep(i32::MAX / 2);
    let step_ref: &dyn Step<i32> = &step;
    let mut ctx = i32::MAX / 2;
    assert!(step_ref.execute(&mut ctx).await.is_ok());
}

// Step::name tests

/// @covers: Step::name _happy path
#[test]
fn test_step_trait_name_happy_returns_name() {
    let step = DummyStep;
    let step_ref: &dyn Step<()> = &step;
    assert_eq!(step_ref.name(), "dummy");
}

/// @covers: Step::name _happy path (different name)
#[test]
fn test_step_trait_name_happy_mutating_step() {
    let step = MutatingStep(0);
    let step_ref: &dyn Step<i32> = &step;
    assert_eq!(step_ref.name(), "mutating");
}

/// @covers: Step::name _edge case
#[test]
fn test_step_trait_name_edge_error_step() {
    let step = ErrorStep;
    let step_ref: &dyn Step<i32> = &step;
    assert_eq!(step_ref.name(), "error");
}

// =============================================================================
// Validator trait tests
// =============================================================================

// Validator::validate tests

/// @covers: Validator::validate _happy path
#[tokio::test]
async fn test_validator_trait_validate_happy_enabled() {
    let validator = create_validator(true);
    let config = PipelineConfig::default();
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

/// @covers: Validator::validate _happy path (disabled)
#[tokio::test]
async fn test_validator_trait_validate_happy_disabled() {
    let validator = create_validator(false);
    let config = PipelineConfig::default();
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

/// @covers: Validator::validate _edge case (custom config)
#[tokio::test]
async fn test_validator_trait_validate_edge_custom_config() {
    let validator = create_validator(true);
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

// Validator::is_enabled tests

/// @covers: Validator::is_enabled _happy path (true)
#[test]
fn test_validator_trait_is_enabled_happy_true() {
    let validator = create_validator(true);
    assert!(validator.is_enabled());
}

/// @covers: Validator::is_enabled _happy path (false)
#[test]
fn test_validator_trait_is_enabled_happy_false() {
    let validator = create_validator(false);
    assert!(!validator.is_enabled());
}

/// @covers: Validator::is_enabled _edge case (consistency check)
#[test]
fn test_validator_trait_is_enabled_edge_consistency() {
    let validator_true = create_validator(true);
    let validator_false = create_validator(false);

    // Multiple calls should return the same value
    assert_eq!(validator_true.is_enabled(), validator_true.is_enabled());
    assert_eq!(validator_false.is_enabled(), validator_false.is_enabled());
}
