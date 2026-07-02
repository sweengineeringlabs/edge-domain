//! Comprehensive API trait function test coverage.
//!
//! Tests for Pipeline and Validator trait methods with happy, error, and edge scenarios.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ConfigValidationRequest, ContextMutationRequest, EnablementRequest, EnablementResponse,
    Pipeline, PipelineConfig, PipelineConfigLookupRequest, PipelineConfigResponse,
    PipelineEmptinessRequest, PipelineError, Step, StepCountRequest, StepCountResponse, StepError,
    StepNameRequest, StepNameResponse, Validator,
};
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Test Doubles
// =============================================================================

#[derive(Clone)]
struct TestPassStep;

#[async_trait::async_trait]
impl Step for TestPassStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "test-pass".to_string(),
        })
    }
}

struct TestPipeline {
    steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>>,
    config: PipelineConfig,
}

impl Service for TestPipeline {
    type Request = i32;
    type Response = i32;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "test-pipeline".to_string(),
        })
    }
    fn execute(&self, ctx: i32) -> BoxFuture<'_, Result<i32, ServiceError>> {
        Box::pin(async move { Ok(ctx) })
    }
}

#[async_trait::async_trait]
impl Pipeline for TestPipeline {
    type Ctx = i32;
    type E = String;

    async fn run(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), PipelineError<String>> {
        let ctx = req.ctx;
        for step in &self.steps {
            let step_name = step.name(StepNameRequest).expect("must succeed").name;
            step.execute(ContextMutationRequest { ctx })
                .await
                .map_err(|e| {
                    PipelineError::StepFailed(StepError {
                        step_name,
                        cause: e,
                    })
                })?;
        }
        Ok(())
    }

    fn step_count(
        &self,
        _req: StepCountRequest,
    ) -> Result<StepCountResponse, PipelineError<String>> {
        Ok(StepCountResponse {
            count: self.steps.len(),
        })
    }

    fn config(
        &self,
        _req: PipelineConfigLookupRequest,
    ) -> Result<PipelineConfigResponse, PipelineError<String>> {
        Ok(PipelineConfigResponse {
            config: self.config.clone(),
        })
    }
}

struct TestValidator {
    enabled: bool,
}

#[async_trait::async_trait]
impl Validator for TestValidator {
    async fn validate(&self, _req: ConfigValidationRequest) -> Result<(), PipelineError<String>> {
        if self.enabled {
            Ok(())
        } else {
            Err(PipelineError::ConfigError(
                "validation disabled".to_string(),
            ))
        }
    }

    fn is_enabled(
        &self,
        _req: EnablementRequest,
    ) -> Result<EnablementResponse, PipelineError<String>> {
        Ok(EnablementResponse {
            enabled: self.enabled,
        })
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
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[test]
fn test_step_count_with_steps_happy() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep), Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        2
    );
}

#[test]
fn test_step_count_many_steps_edge() {
    let steps: Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> = (0..100)
        .map(|_| Arc::new(TestPassStep) as Arc<dyn Step<Ctx = i32, ExecutionError = String>>)
        .collect();
    let pipeline = TestPipeline {
        steps,
        config: PipelineConfig::default(),
    };
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        100
    );
}

#[test]
fn test_step_count_consistency_error() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    let count1 = pipeline
        .step_count(StepCountRequest)
        .expect("must succeed")
        .count;
    let count2 = pipeline
        .step_count(StepCountRequest)
        .expect("must succeed")
        .count;
    assert_eq!(count1, count2);
    assert_eq!(count1, 1);
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
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_is_empty_with_steps_happy() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_is_empty_consistency_edge() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
    let first_call = pipeline
        .is_empty(PipelineEmptinessRequest)
        .expect("must succeed")
        .empty;
    let second_call = pipeline
        .is_empty(PipelineEmptinessRequest)
        .expect("must succeed")
        .empty;
    assert_eq!(first_call, second_call);
}

#[test]
fn test_is_empty_consistency_implies_step_count_error() {
    let pipeline = TestPipeline {
        steps: vec![Arc::new(TestPassStep)],
        config: PipelineConfig::default(),
    };
    let is_empty = pipeline
        .is_empty(PipelineEmptinessRequest)
        .expect("must succeed")
        .empty;
    let step_count = pipeline
        .step_count(StepCountRequest)
        .expect("must succeed")
        .count;
    assert_eq!(is_empty, step_count == 0);
}

// =============================================================================
// Pipeline::config Tests
// =============================================================================

#[test]
fn test_config_default_happy() {
    let config = PipelineConfig::default();
    let pipeline = TestPipeline {
        steps: vec![],
        config,
    };
    let cfg = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(cfg.timeout_per_step, None);
    assert!(!cfg.emit_lifecycle_events);
    assert!(cfg.abort_on_error);
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
        config,
    };
    let cfg = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(cfg.timeout_per_step, Some(Duration::from_secs(5)));
    assert!(cfg.emit_lifecycle_events);
    assert!(!cfg.abort_on_error);
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
    let ref1 = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    let ref2 = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(ref1.timeout_per_step, ref2.timeout_per_step);
}

#[test]
fn test_config_consistency_error() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline = TestPipeline {
        steps: vec![],
        config,
    };
    let cfg1 = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    let cfg2 = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
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
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

#[test]
fn test_is_enabled_disabled_happy() {
    let validator = TestValidator { enabled: false };
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

#[test]
fn test_is_enabled_consistency_edge() {
    let validator = TestValidator { enabled: true };
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    let first_call = validator
        .is_enabled(EnablementRequest)
        .expect("must succeed")
        .enabled;
    let second_call = validator
        .is_enabled(EnablementRequest)
        .expect("must succeed")
        .enabled;
    assert_eq!(first_call, second_call);
}

#[test]
fn test_is_enabled_disabled_error() {
    let validator = TestValidator { enabled: false };
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

// =============================================================================
// Validator::validate Tests
// =============================================================================

#[tokio::test]
async fn test_validate_enabled_happy() {
    let validator = TestValidator { enabled: true };
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_validate_disabled_error() {
    let validator = TestValidator { enabled: false };
    let config = PipelineConfig::default();
    let result = validator.validate(ConfigValidationRequest { config }).await;
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
    assert!(validator
        .validate(ConfigValidationRequest { config: config1 })
        .await
        .is_ok());
    assert!(validator
        .validate(ConfigValidationRequest { config: config2 })
        .await
        .is_ok());
}
