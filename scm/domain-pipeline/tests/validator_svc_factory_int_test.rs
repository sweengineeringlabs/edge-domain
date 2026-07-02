//! Integration tests — `ValidatorSvc` construction surface.
//! @covers ValidatorSvc::create, ValidatorSvc::create_shared
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_pipeline::{
    ConfigValidationRequest, EnablementRequest, PipelineConfig, PipelineError, ValidatorSvc,
};

// ── ValidatorSvc::create ──────────────────────────────────────────────────────

/// @covers: create
#[tokio::test]
async fn test_create_enabled_valid_config_passes_happy() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

/// @covers: create
#[tokio::test]
async fn test_create_enabled_invalid_config_returns_error() {
    let validator = ValidatorSvc::create(true);
    let config = PipelineConfig {
        abort_on_error: false,
        ..Default::default()
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::ConfigError(_))));
}

/// @covers: create
#[tokio::test]
async fn test_create_disabled_skips_validation_edge() {
    let validator = ValidatorSvc::create(false);
    let config = PipelineConfig {
        abort_on_error: false,
        ..Default::default()
    };
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

// ── ValidatorSvc::create_shared ───────────────────────────────────────────────

/// @covers: create_shared
#[tokio::test]
async fn test_create_shared_enabled_valid_config_passes_happy() {
    let validator = ValidatorSvc::create_shared(true);
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
}

/// @covers: create_shared
#[tokio::test]
async fn test_create_shared_enabled_invalid_config_returns_error() {
    let validator = ValidatorSvc::create_shared(true);
    let config = PipelineConfig {
        abort_on_error: false,
        ..Default::default()
    };
    let result = validator.validate(ConfigValidationRequest { config }).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::ConfigError(_))));
}

/// @covers: create_shared
#[tokio::test]
async fn test_create_shared_arc_is_cloneable_edge() {
    let validator = ValidatorSvc::create_shared(true);
    let clone = Arc::clone(&validator);
    let config = PipelineConfig::default();
    assert!(validator
        .validate(ConfigValidationRequest {
            config: config.clone()
        })
        .await
        .is_ok());
    assert!(clone
        .validate(ConfigValidationRequest { config })
        .await
        .is_ok());
    assert_eq!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled,
        clone
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}
