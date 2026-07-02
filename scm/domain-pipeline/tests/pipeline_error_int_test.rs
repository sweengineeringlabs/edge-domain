//! Integration tests for the [`PipelineError`] type.
//!
//! @covers PipelineError
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fmt;
use std::fmt::Display;

use edge_domain_pipeline::{PipelineError, StepError};

/// @covers: general
#[test]
fn test_error_step_failed_formats_with_message() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "x".to_string(),
        cause: "validation failed".to_string(),
    });
    let msg = format!("{}", err);
    assert!(msg.contains("validation failed"));
}

/// @covers: general
#[test]
fn test_error_step_timeout_formats_readable() {
    let err: PipelineError<String> = PipelineError::StepTimeout {
        step_name: "x".to_string(),
    };
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}

/// @covers: general
#[test]
fn test_error_config_error_formats_with_message() {
    let err: PipelineError<String> = PipelineError::ConfigError("invalid timeout".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("invalid timeout"));
}

/// @covers: general
#[test]
fn test_error_is_display_impl() {
    let err: Box<dyn Display> = Box::new(PipelineError::<String>::StepFailed(StepError {
        step_name: "x".to_string(),
        cause: "test".to_string(),
    }));
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}

// std::error::Error requires E: std::error::Error + Send + 'static.
// String does not satisfy this bound, so we need a proper error type.
#[derive(Debug)]
struct AnyError(String);

impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AnyError {}

/// @covers: general
#[test]
fn test_error_is_std_error() {
    let err: Box<dyn std::error::Error> =
        Box::new(PipelineError::<AnyError>::StepFailed(StepError {
            step_name: "test".to_string(),
            cause: AnyError("test".to_string()),
        }));
    assert!(!err.to_string().is_empty());
}

// Additional coverage for step_name field
#[test]
fn test_error_step_failed_preserves_step_name() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "my-step".to_string(),
        cause: "oops".to_string(),
    });
    match err {
        PipelineError::StepFailed(e) => {
            assert_eq!(e.step_name, "my-step");
            assert_eq!(e.cause, "oops");
        }
        _ => panic!("expected StepFailed"),
    }
}

#[test]
fn test_error_timeout_preserves_step_name() {
    let err: PipelineError<String> = PipelineError::StepTimeout {
        step_name: "slow-step".to_string(),
    };
    match err {
        PipelineError::StepTimeout { step_name } => assert_eq!(step_name, "slow-step"),
        _ => panic!("expected StepTimeout"),
    }
}

#[test]
fn test_error_unknown_step_happy() {
    let err: PipelineError<String> = PipelineError::UnknownStep("ghost".to_string());
    match err {
        PipelineError::UnknownStep(name) => assert_eq!(name, "ghost"),
        _ => panic!("expected UnknownStep"),
    }
}
