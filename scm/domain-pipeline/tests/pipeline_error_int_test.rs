//! Integration tests for the [`PipelineError`] type.
//!
//! @covers PipelineError

use edge_domain_pipeline::PipelineError;
use std::fmt::Display;

/// @covers: general
#[test]
fn test_error_step_failed_formats_with_message() {
    let err = PipelineError::StepFailed("validation failed".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("validation failed"));
}

/// @covers: general
#[test]
fn test_error_step_timeout_formats_readable() {
    let err = PipelineError::StepTimeout;
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}

/// @covers: general
#[test]
fn test_error_config_error_formats_with_message() {
    let err = PipelineError::ConfigError("invalid timeout".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("invalid timeout"));
}

/// @covers: general
#[test]
fn test_error_is_display_impl() {
    let err: Box<dyn Display> = Box::new(PipelineError::StepFailed("test".to_string()));
    let msg = format!("{}", err);
    assert!(msg.len() > 0);
}

/// @covers: general
#[test]
fn test_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(PipelineError::StepFailed("test".to_string()));
    assert!(!err.to_string().is_empty());
}
