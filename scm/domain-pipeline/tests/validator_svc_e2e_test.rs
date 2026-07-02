//! E2E tests for validator service (SAF layer).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ConfigValidationRequest, EnablementRequest, PipelineConfig, ValidatorSvc,
};

#[test]
fn test_create_validator_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

#[test]
fn test_create_validator_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

#[tokio::test]
async fn test_create_validator_happy_enabled_validates() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_disabled_skips_validation() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_multiple_validations() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest {
            config: config.clone()
        })
        .await
        .is_ok());
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_create_validator_happy_disabled_consistent() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let result1 = validator
        .validate(ConfigValidationRequest {
            config: config.clone(),
        })
        .await;
    let result2 = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
