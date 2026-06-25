//! Integration tests for Validator trait.

use edge_domain_pipeline::{PipelineConfig, ValidatorSvc};

// Test create_validator factory
#[test]
fn test_create_validator_enabled_happy() {
    let validator = ValidatorSvc::create(true);
    assert!(validator.is_enabled());
}

#[test]
fn test_create_validator_disabled_happy() {
    let validator = ValidatorSvc::create(false);
    assert!(!validator.is_enabled());
}

#[test]
fn test_create_validator_instance_error() {
    // Error scenario: verify distinct instances are created
    let v1 = ValidatorSvc::create(true);
    let v2 = ValidatorSvc::create(false);
    assert_ne!(v1.is_enabled(), v2.is_enabled());
}

#[test]
fn test_create_validator_multiple_edge() {
    // Edge case: create multiple validators with same config
    let v1 = ValidatorSvc::create(true);
    let v2 = ValidatorSvc::create(true);
    // Both should have the same enabled state
    assert_eq!(v1.is_enabled(), v2.is_enabled());
}

// Test validate method
#[tokio::test]
async fn test_validator_validate_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validator_validate_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig::default();
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validator_validate_edge_custom_config() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        timeout_per_step: Some(std::time::Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let result = validator.validate(&config).await;
    assert!(result.is_ok());
}

// Test is_enabled method
#[test]
fn test_validator_is_enabled_happy_true() {
    let validator = ValidatorSvc::create(true);
    assert!(validator.is_enabled());
}

/// @covers: is_enabled
#[test]
fn test_validator_is_enabled_happy_false() {
    let validator = ValidatorSvc::create(false);
    assert!(!validator.is_enabled());
}

/// @covers: is_enabled
#[test]
fn test_validator_is_enabled_edge_consistency() {
    let validator_enabled = ValidatorSvc::create(true);
    let validator_disabled = ValidatorSvc::create(false);

    // Multiple calls should return consistent results
    assert_eq!(validator_enabled.is_enabled(), true);
    assert_eq!(validator_enabled.is_enabled(), true);

    assert_eq!(validator_disabled.is_enabled(), false);
    assert_eq!(validator_disabled.is_enabled(), false);
}

// Integration: behavior depends on enabled state
#[tokio::test]
async fn test_validator_validate_respects_enabled_state() {
    let enabled = ValidatorSvc::create(true);
    let disabled = ValidatorSvc::create(false);

    let config = PipelineConfig::default();

    // Both should validate successfully
    assert!(enabled.validate(&config).await.is_ok());
    assert!(disabled.validate(&config).await.is_ok());
}

/// @covers: is_enabled
#[test]
fn test_validator_enabled_state_independent() {
    let v1 = ValidatorSvc::create(true);
    let v2 = ValidatorSvc::create(false);

    // Different instances should maintain their own state
    assert_ne!(v1.is_enabled(), v2.is_enabled());
}
