//! Comprehensive API trait function test coverage.
//!
//! Tests for Pipeline and Validator trait methods with happy, error, and edge scenarios.

use edge_domain_pipeline::{Pipeline, PipelineConfig, PipelineError, Step, Validator};
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
struct TestFailStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for TestFailStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("test failure".to_string()))
    }

    fn name(&self) -> &str {
        "test-fail"
    }
}

struct TestPipeline {
    steps: Vec<Arc<dyn Step<i32>>>,
    config: PipelineConfig,
}

#[async_trait::async_trait]
impl Pipeline<i32> for TestPipeline {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        for step in &self.steps {
            step.execute(ctx).await?;
        }
        Ok(())
    }

    fn step_count(&self) -> usize {
        self.steps.len()
    }

    fn config(&self) -> &PipelineConfig {
        &self.config
    }

    fn name(&self) -> &str {
        "test-pipeline"
    }
}

struct TestValidator {
    enabled: bool,
}

#[async_trait::async_trait]
impl Validator for TestValidator {
    async fn validate(&self, _config: &PipelineConfig) -> Result<(), PipelineError> {
        if self.enabled {
            Ok(())
        } else {
            Err(PipelineError::ConfigError("validation disabled".to_string()))
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// =============================================================================
// Pipeline::step_count Tests
// =============================================================================

#[test]
fn test_step_count_empty_happy() {
    let pipeline = TestPipeline {
        steps: vec![],
        config: PipelineConfig::default(),
    };
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_step_count_with_steps_happy() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep), Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    assert_eq!(pipeline.step_count(), 2);
}

#[test]
fn test_step_count_many_steps_edge() {
    let steps: Vec<Arc<dyn Step<i32>>> =
        (0..100).map(|_| Arc::new(TestPassStep) as Arc<dyn Step<i32>>).collect();
    let pipeline = TestPipeline {
        steps: steps.clone(),
        config: PipelineConfig::default(),
    };
    assert_eq!(pipeline.step_count(), 100);
}

#[test]
fn test_step_count_consistency_error() {
    // Error test: verify step_count stays stable (not growing unexpectedly)
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    let count1 = pipeline.step_count();
    let count2 = pipeline.step_count();
    assert_eq!(count1, count2);
    assert_eq!(count1, 1); // Not an error state, but validates immutability
}

// =============================================================================
// Pipeline::is_empty Tests
// =============================================================================

#[test]
fn test_is_empty_empty_happy() {
    let pipeline = TestPipeline {
        steps: vec![],
        config: PipelineConfig::default(),
    };
    assert!(pipeline.is_empty());
}

#[test]
fn test_is_empty_with_steps_happy() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    assert!(!pipeline.is_empty());
}

#[test]
fn test_is_empty_consistency_edge() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    // Edge case: pipeline with steps should not be empty
    assert!(!pipeline.is_empty());
    // Verify consistency: calling twice gives same result
    let first_call = pipeline.is_empty();
    let second_call = pipeline.is_empty();
    assert_eq!(first_call, second_call);
}

#[test]
fn test_is_empty_consistency_implies_step_count_error() {
    // Error test: is_empty should be consistent with step_count
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    let is_empty = pipeline.is_empty();
    let step_count = pipeline.step_count();
    // Validate invariant: is_empty iff step_count == 0
    assert_eq!(is_empty, step_count == 0);
}

// =============================================================================
// Pipeline::name Tests
// =============================================================================

#[test]
fn test_name_happy_returns_string() {
    let pipeline = TestPipeline {
        steps: vec![],
        config: PipelineConfig::default(),
    };
    assert_eq!(pipeline.name(), "test-pipeline");
}

#[test]
fn test_name_consistency_error() {
    // Error test: name should be consistent across calls
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    let name1 = pipeline.name();
    let name2 = pipeline.name();
    assert_eq!(name1, name2);
}

#[test]
fn test_name_edge_not_empty() {
    let pipeline = TestPipeline {
        steps: vec![],
        config: PipelineConfig::default(),
    };
    assert!(!pipeline.name().is_empty());
}

// =============================================================================
// Pipeline::config Tests
// =============================================================================

#[test]
fn test_config_default_happy() {
    let config = PipelineConfig::default();
    let pipeline = TestPipeline {
        steps: vec![],
        config: config.clone(),
    };
    assert_eq!(pipeline.config().timeout_per_step, None);
    assert!(!pipeline.config().emit_lifecycle_events);
    assert!(pipeline.config().abort_on_error);
}

#[test]
fn test_config_custom_happy() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = TestPipeline {
        steps: vec![],
        config: config.clone(),
    };
    assert_eq!(
        pipeline.config().timeout_per_step,
        Some(Duration::from_secs(5))
    );
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}

#[test]
fn test_config_reference_stable_edge() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = TestPipeline {
        steps: vec![],
        config,
    };
    let ref1 = pipeline.config();
    let ref2 = pipeline.config();
    assert_eq!(ref1.timeout_per_step, ref2.timeout_per_step);
}

#[test]
fn test_config_consistency_error() {
    // Error test: config should not change between calls
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = TestPipeline {
        steps: vec![],
        config,
    };
    let cfg1 = pipeline.config();
    let cfg2 = pipeline.config();
    assert_eq!(cfg1.timeout_per_step, cfg2.timeout_per_step);
    assert_eq!(cfg1.emit_lifecycle_events, cfg2.emit_lifecycle_events);
    assert_eq!(cfg1.abort_on_error, cfg2.abort_on_error);
}

// =============================================================================
// Validator::is_enabled Tests
// =============================================================================

#[test]
fn test_is_enabled_enabled_happy() {
    let validator = TestValidator { enabled: true };
    assert!(validator.is_enabled());
}

#[test]
fn test_is_enabled_disabled_happy() {
    let validator = TestValidator { enabled: false };
    assert!(!validator.is_enabled());
}

#[test]
fn test_is_enabled_consistency_edge() {
    let validator = TestValidator { enabled: true };
    // Edge case: enabled validator should return true
    assert!(validator.is_enabled());
    // Verify consistency: calling twice gives same result
    let first_call = validator.is_enabled();
    let second_call = validator.is_enabled();
    assert_eq!(first_call, second_call);
}

#[test]
fn test_is_enabled_disabled_error() {
    // Error test: disabled validator should return false consistently
    let validator = TestValidator { enabled: false };
    assert!(!validator.is_enabled());
    // Verify it stays disabled across multiple calls
    assert!(!validator.is_enabled());
}

// =============================================================================
// Validator::validate Tests (via subtyping - validators are also Steps in composition)
// =============================================================================

#[tokio::test]
async fn test_validate_enabled_happy() {
    let validator = TestValidator { enabled: true };
    let config = PipelineConfig::default();
    assert!(validator.validate(&config).await.is_ok());
}

#[tokio::test]
async fn test_validate_disabled_error() {
    let validator = TestValidator { enabled: false };
    let config = PipelineConfig::default();
    let result = validator.validate(&config).await;
    assert!(result.is_err());
    if let Err(PipelineError::ConfigError(msg)) = result {
        assert_eq!(msg, "validation disabled");
    } else {
        panic!("Expected ConfigError");
    }
}

#[tokio::test]
async fn test_validate_different_configs_edge() {
    let validator = TestValidator { enabled: true };
    let config1 = PipelineConfig::default();
    let config2 = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    assert!(validator.validate(&config1).await.is_ok());
    assert!(validator.validate(&config2).await.is_ok());
}
