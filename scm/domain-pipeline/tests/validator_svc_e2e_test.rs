//! E2E tests for validator service (SAF layer).

use edge_domain_pipeline::{PipelineConfig, ValidatorSvc};

#[test]
fn test_create_validator_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    assert!(validator.is_enabled());
}

#[test]
fn test_create_validator_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    assert!(!validator.is_enabled());
}

#[tokio::test]
async fn test_create_validator_happy_enabled_validates() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    assert!(validator.validate(&config).await.is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_disabled_skips_validation() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    assert!(validator.validate(&config).await.is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_multiple_validations() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    assert!(validator.validate(&config).await.is_ok());
    assert!(validator.validate(&config).await.is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_disabled_consistent() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let result1 = validator.validate(&config).await;
    let result2 = validator.validate(&config).await;
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
