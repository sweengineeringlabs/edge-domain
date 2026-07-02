//! @covers Validator trait
//! Comprehensive trait implementation tests for Validator interface.
//! Ensures all trait methods have proper test coverage across happy, error, and edge paths.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    BuilderValidationRequest, ConfigValidationRequest, EnablementRequest, EnablementResponse,
    PipelineBuilder, PipelineConfig, PipelineError, ValidatorSvc,
};
use std::time::Duration;

// Validator::validate tests

/// Test validate returns ok for enabled validator with default config
#[tokio::test]
async fn test_validator_validate_enabled_happy() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate returns ok for disabled validator with default config
#[tokio::test]
async fn test_validator_validate_disabled_happy() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig::default();
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate with custom config including timeout
#[tokio::test]
async fn test_validator_validate_with_timeout_happy() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate with all options enabled
#[tokio::test]
async fn test_validator_validate_all_options_enabled_edge() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate with all options disabled
#[tokio::test]
async fn test_validator_validate_all_options_disabled_error() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate with very large timeout
#[tokio::test]
async fn test_validator_validate_large_timeout_edge() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(3600)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_ok());
}

/// Test validate is consistent across multiple calls
#[tokio::test]
async fn test_validator_validate_consistency_error() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();

    let result1 = validator
        .validate(ConfigValidationRequest {
            config: config.clone(),
        })
        .await;
    let result2 = validator.validate(ConfigValidationRequest { config }).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

// Validator::is_enabled tests

/// Test is_enabled returns true when created with true
#[test]
fn test_validator_is_enabled_true_happy() {
    let validator = ValidatorSvc::create(true);
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// Test is_enabled returns false when created with false
#[test]
fn test_validator_is_enabled_false_happy() {
    let validator = ValidatorSvc::create(false);
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// Test is_enabled consistency across multiple calls
#[test]
fn test_validator_is_enabled_consistency_happy() {
    let validator_enabled = ValidatorSvc::create(true);
    assert!(
        validator_enabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        validator_enabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );

    let validator_disabled = ValidatorSvc::create(false);
    assert!(
        !validator_disabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !validator_disabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// Test is_enabled distinguishes between instances
#[test]
fn test_validator_is_enabled_instances_error() {
    let enabled = ValidatorSvc::create(true);
    let disabled = ValidatorSvc::create(false);

    assert_ne!(
        enabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled,
        disabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

// Validator::validate_builder tests
//
// validate_builder has `where Self: Sized` (keeps Validator dyn-compatible), so
// it must be tested through a concrete type, not Box<dyn Validator>.

use edge_domain_pipeline::Validator;

struct ConcreteValidator {
    enabled: bool,
}

#[async_trait::async_trait]
impl Validator for ConcreteValidator {
    async fn validate(&self, req: ConfigValidationRequest) -> Result<(), PipelineError<String>> {
        if !self.enabled {
            return Ok(());
        }
        if req.config.abort_on_error {
            Ok(())
        } else {
            Err(PipelineError::ConfigError(
                "abort_on_error must be true".to_string(),
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

/// validate_builder delegates to validate — valid config succeeds
#[tokio::test]
async fn test_validate_builder_happy_valid_config_succeeds() {
    let validator = ConcreteValidator { enabled: true };
    let builder = PipelineBuilder::<i32, String>::new();
    assert!(validator
        .validate_builder(BuilderValidationRequest { builder: &builder })
        .await
        .is_ok());
}

/// validate_builder delegates to validate — invalid config (abort_on_error=false) fails
#[tokio::test]
async fn test_validate_builder_error_invalid_config_fails() {
    let validator = ConcreteValidator { enabled: true };
    let builder = PipelineBuilder::<i32, String>::new().abort_on_error(false);
    match validator
        .validate_builder(BuilderValidationRequest { builder: &builder })
        .await
    {
        Err(PipelineError::ConfigError(_)) => {}
        other => panic!("expected ConfigError, got {:?}", other),
    }
}

/// validate_builder passes when validator is disabled regardless of config
#[tokio::test]
async fn test_validate_builder_edge_disabled_validator_passes_any_config() {
    let validator = ConcreteValidator { enabled: false };
    let builder = PipelineBuilder::<i32, String>::new().abort_on_error(false);
    assert!(validator
        .validate_builder(BuilderValidationRequest { builder: &builder })
        .await
        .is_ok());
}

/// Test multiple enabled validators
#[test]
fn test_validator_is_enabled_multiple_enabled_edge() {
    let v1 = ValidatorSvc::create(true);
    let v2 = ValidatorSvc::create(true);
    let v3 = ValidatorSvc::create(true);

    assert!(
        v1.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        v2.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        v3.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// Test multiple disabled validators
#[test]
fn test_validator_is_enabled_multiple_disabled_edge() {
    let v1 = ValidatorSvc::create(false);
    let v2 = ValidatorSvc::create(false);
    let v3 = ValidatorSvc::create(false);

    assert!(
        !v1.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !v2.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !v3.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

// Integration tests combining validate and is_enabled

/// Test that enabled state is reflected in validation behavior
#[tokio::test]
async fn test_validator_enabled_affects_behavior_happy() {
    let enabled = ValidatorSvc::create(true);
    let disabled = ValidatorSvc::create(false);

    let config = PipelineConfig::default();

    assert!(
        enabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(enabled
        .validate(ConfigValidationRequest {
            config: config.clone()
        })
        .await
        .is_ok());

    assert!(
        !disabled
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(disabled
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

/// Test state independence across validators
#[test]
fn test_validator_state_independence_error() {
    let v1 = ValidatorSvc::create(true);
    let v2 = ValidatorSvc::create(false);
    let v3 = ValidatorSvc::create(true);

    assert!(
        v1.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        !v2.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert!(
        v3.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
    assert_ne!(
        v1.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled,
        v2.is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

/// Test validator behavior with complex config
#[tokio::test]
async fn test_validator_complex_config_edge() {
    let validator = ValidatorSvc::create(true);

    let configs = vec![
        PipelineConfig {
            timeout_per_step: Some(Duration::from_millis(100)),
            emit_lifecycle_events: true,
            abort_on_error: true,
        },
        PipelineConfig {
            timeout_per_step: Some(Duration::from_secs(60)),
            emit_lifecycle_events: false,
            abort_on_error: true,
        },
        PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: true,
            abort_on_error: true,
        },
    ];

    for config in configs {
        assert!(validator
            .validate(ConfigValidationRequest { config })
            .await
            .is_ok());
    }
}
