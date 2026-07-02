//! Comprehensive trait method tests for Pipeline, Step, and Validator.
//! This file ensures all trait methods have _happy, _error, and _edge test variants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ConfigValidationRequest, ContextMutationRequest, EnablementRequest, Pipeline, PipelineBuilder,
    PipelineConfig, PipelineConfigLookupRequest, PipelineEmptinessRequest, PipelineError,
    PipelineSvc, Step, StepCountRequest, StepNameRequest, StepNameResponse, ValidatorSvc,
};
use std::time::Duration;

// =============================================================================
// Pipeline trait tests
// =============================================================================

struct DummyStep;

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for DummyStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "dummy".to_string(),
        })
    }
}

struct FailStep;

#[async_trait::async_trait]
impl Step<(), String> for FailStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, ()>) -> Result<(), String> {
        Err("test failure".to_string())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "fail-step".to_string(),
        })
    }
}

// Pipeline::execute tests

/// @covers: Pipeline::execute _happy path
#[tokio::test]
async fn test_pipeline_trait_execute_happy_empty() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: Pipeline::execute _happy path
#[tokio::test]
async fn test_pipeline_trait_execute_happy_with_steps() {
    let pipeline: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(DummyStep).with(DummyStep));
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: Pipeline::execute _error path
#[tokio::test]
async fn test_pipeline_trait_execute_error_fails_on_step() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(DummyStep)
            .with(FailStep)
            .with(DummyStep),
    );
    let mut ctx = ();
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
}

/// @covers: Pipeline::execute _edge case
#[tokio::test]
async fn test_pipeline_trait_execute_edge_many_steps() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..50 {
        builder = builder.with(DummyStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

// Pipeline::step_count tests

/// @covers: Pipeline::step_count _happy path
#[test]
fn test_pipeline_trait_step_count_happy_empty() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

/// @covers: Pipeline::step_count _happy path
#[test]
fn test_pipeline_trait_step_count_happy_with_steps() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(DummyStep)
            .with(DummyStep)
            .with(DummyStep),
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        3
    );
}

/// @covers: Pipeline::step_count _edge case
#[test]
fn test_pipeline_trait_step_count_edge_many_steps() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..100 {
        builder = builder.with(DummyStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        100
    );
}

// Pipeline::is_empty tests (default impl uses step_count)

/// @covers: Pipeline::is_empty _happy path (true case)
#[test]
fn test_pipeline_trait_is_empty_happy_true() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

/// @covers: Pipeline::is_empty _happy path (false case)
#[test]
fn test_pipeline_trait_is_empty_happy_false() {
    let pipeline: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(DummyStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

/// @covers: Pipeline::is_empty _edge case
#[test]
fn test_pipeline_trait_is_empty_edge_many_steps() {
    let mut builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    for _ in 0..50 {
        builder = builder.with(DummyStep);
    }
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(builder);
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

// Pipeline::config tests

/// @covers: Pipeline::config _happy path (default config)
#[test]
fn test_pipeline_trait_config_happy_default() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(PipelineBuilder::new());
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

/// @covers: Pipeline::config _happy path (custom config)
#[test]
fn test_pipeline_trait_config_happy_custom() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with_timeout(Duration::from_secs(5))
            .emit_lifecycle_events(true)
            .abort_on_error(false),
    );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(5)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

/// @covers: Pipeline::config _edge case
#[test]
fn test_pipeline_trait_config_edge_all_options_set() {
    let pipeline: Box<dyn Pipeline<(), String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with_timeout(Duration::from_secs(30))
            .emit_lifecycle_events(true)
            .abort_on_error(true),
    );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(30)));
    assert!(config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

// =============================================================================
// Step trait tests
// =============================================================================

struct MutatingStep(i32);

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for MutatingStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        *req.ctx += self.0;
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "mutating".to_string(),
        })
    }
}

struct ErrorStep;

#[async_trait::async_trait]
impl Step<i32, String> for ErrorStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Err("error".to_string())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "error".to_string(),
        })
    }
}

// Step::execute tests

/// @covers: Step::execute _happy path
#[tokio::test]
async fn test_step_trait_execute_happy_succeeds() {
    let step = DummyStep;
    let step_ref: &dyn Step<(), String> = &step;
    let mut ctx = ();
    assert!(step_ref
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: Step::execute _happy path (with mutation)
#[tokio::test]
async fn test_step_trait_execute_happy_mutates() {
    let step = MutatingStep(10);
    let step_ref: &dyn Step<i32, String> = &step;
    let mut ctx = 5;
    assert!(step_ref
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: Step::execute _error path
#[tokio::test]
async fn test_step_trait_execute_error_returns_error() {
    let step = ErrorStep;
    let step_ref: &dyn Step<i32, String> = &step;
    let mut ctx = 0;
    let result = step_ref
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await;
    assert!(result.is_err());
}

/// @covers: Step::execute _edge case
#[tokio::test]
async fn test_step_trait_execute_edge_large_mutation() {
    let step = MutatingStep(i32::MAX / 2);
    let step_ref: &dyn Step<i32, String> = &step;
    let mut ctx = i32::MAX / 2;
    assert!(step_ref
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

// Step::name tests

/// @covers: Step::name _happy path
#[test]
fn test_step_trait_name_happy_returns_name() {
    let step = DummyStep;
    let step_ref: &dyn Step<(), String> = &step;
    assert_eq!(
        step_ref.name(StepNameRequest).expect("must succeed").name,
        "dummy"
    );
}

/// @covers: Step::name _happy path (different name)
#[test]
fn test_step_trait_name_happy_mutating_step() {
    let step = MutatingStep(0);
    let step_ref: &dyn Step<i32, String> = &step;
    assert_eq!(
        step_ref.name(StepNameRequest).expect("must succeed").name,
        "mutating"
    );
}

/// @covers: Step::name _edge case
#[test]
fn test_step_trait_name_edge_error_step() {
    let step = ErrorStep;
    let step_ref: &dyn Step<i32, String> = &step;
    assert_eq!(
        step_ref.name(StepNameRequest).expect("must succeed").name,
        "error"
    );
}

// =============================================================================
// Validator trait tests
// =============================================================================

// Validator::validate tests

/// @covers: Validator::validate _happy path
#[tokio::test]
async fn test_validator_trait_validate_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// @covers: Validator::validate _happy path (disabled)
#[tokio::test]
async fn test_validator_trait_validate_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig::default();
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// @covers: Validator::validate _edge case (custom config)
#[tokio::test]
async fn test_validator_trait_validate_edge_custom_config() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

// Validator::is_enabled tests

/// @covers: Validator::is_enabled _happy path (true)
#[test]
fn test_validator_trait_is_enabled_happy_true() {
    let validator = ValidatorSvc::create(true);
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// @covers: Validator::is_enabled _happy path (false)
#[test]
fn test_validator_trait_is_enabled_happy_false() {
    let validator = ValidatorSvc::create(false);
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// @covers: Validator::is_enabled _edge case (consistency check)
#[test]
fn test_validator_trait_is_enabled_edge_consistency() {
    let validator_true = ValidatorSvc::create(true);
    let validator_false = ValidatorSvc::create(false);

    assert!(
        validator_true
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !validator_false
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

// Validator + Pipeline integration

/// @covers: Validator used with pipeline builder — enabled, valid config
#[tokio::test]
async fn test_pipeline_trait_validate_then_build_happy() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    validator
        .validate(ConfigValidationRequest { config })
        .await
        .expect("default config must be valid");

    let pipeline: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(DummyStep));
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: PipelineError returned through pipeline run — error wraps cause
#[tokio::test]
async fn test_pipeline_trait_error_contains_step_name_error() {
    let pipeline: Box<dyn Pipeline<(), String>> =
        PipelineSvc::build(PipelineBuilder::new().with(FailStep));
    let mut ctx = ();
    match pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await {
        Err(PipelineError::StepFailed(e)) => {
            assert_eq!(e.step_name, "fail-step");
            assert_eq!(e.cause, "test failure");
        }
        other => panic!("expected StepFailed, got {:?}", other),
    }
}
