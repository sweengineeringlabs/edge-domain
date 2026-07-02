//! Integration tests for the [`StepError`] type (Display and Error trait impls).
//!
//! @covers StepError
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fmt;

use edge_domain_pipeline::StepError;

#[derive(Debug)]
struct SimpleError(String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SimpleError {}

/// @covers: Display
#[test]
fn test_display_happy_formats_step_name_and_cause() {
    let err = StepError {
        step_name: "extract-token".to_string(),
        cause: "missing header".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("extract-token"),
        "Display must include the step name"
    );
    assert!(
        msg.contains("missing header"),
        "Display must include the cause"
    );
}

/// @covers: Display
#[test]
fn test_display_error_empty_step_name() {
    let err = StepError {
        step_name: String::new(),
        cause: "cause".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("cause"),
        "cause must appear in Display even when step name is empty"
    );
}

/// @covers: Display
#[test]
fn test_display_edge_step_name_and_cause_both_present() {
    let err = StepError {
        step_name: "validate-token".to_string(),
        cause: "expired".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("validate-token"));
    assert!(msg.contains("expired"));
}

/// @covers: Error::source
#[test]
fn test_source_happy_returns_cause_when_error_impl() {
    let err = StepError {
        step_name: "step".to_string(),
        cause: SimpleError("the reason".to_string()),
    };
    let source = std::error::Error::source(&err);
    assert!(source.is_some(), "source() must return Some when E: Error");
    assert!(
        source.unwrap().to_string().contains("the reason"),
        "source must expose the cause message"
    );
}

/// @covers: Error::source
#[test]
fn test_source_error_cause_message_is_accessible() {
    let err = StepError {
        step_name: "parse".to_string(),
        cause: SimpleError("unexpected EOF".to_string()),
    };
    let source = std::error::Error::source(&err);
    assert_eq!(
        source.unwrap().to_string(),
        "unexpected EOF",
        "source message must match the original cause exactly"
    );
}

/// @covers: Error::source
#[test]
fn test_source_edge_empty_cause_message() {
    let err = StepError {
        step_name: "check".to_string(),
        cause: SimpleError(String::new()),
    };
    let source = std::error::Error::source(&err);
    assert!(
        source.is_some(),
        "source must be Some even when cause message is empty"
    );
    assert_eq!(source.unwrap().to_string(), "");
}
