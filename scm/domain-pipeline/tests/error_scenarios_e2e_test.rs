//! @covers error handling and edge cases
//! Error scenario tests for PipelineError and error handling.

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, {PipelineError, Pipeline, DefaultPipeline, Step};
use std::sync::Arc;

struct ErrorWithContext(String);

#[async_trait::async_trait]
impl Step<String> for ErrorWithContext {
    async fn execute(&self, ctx: &mut String) -> Result<(), PipelineError> {
        ctx.push_str(&self.0);
        Err(PipelineError::StepFailed("step error".to_string()))
    }

    fn name(&self) -> &str {
        "error-step"
    }
}

// PipelineError variants
#[test]
fn test_error_step_failed_happy() {
    let err = PipelineError::StepFailed("test error".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("test error"));
}

#[test]
fn test_error_step_timeout_happy() {
    let _err = PipelineError::StepTimeout;
}

#[test]
fn test_error_config_error_happy() {
    let err = PipelineError::ConfigError("config issue".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("config issue"));
}

// Error propagation
#[tokio::test]
async fn test_error_propagation_happy_stops_pipeline() {
    let pipeline = create_pipeline(vec![
        Arc::new(ErrorWithContext("partial".to_string())),
    ]);
    let mut ctx = String::new();
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, "partial");
}

#[tokio::test]
async fn test_error_context_mutation_before_error() {
    let pipeline = create_pipeline(vec![
        Arc::new(ErrorWithContext("before".to_string())),
    ]);
    let mut ctx = String::new();
    let _ = Pipeline::execute(&pipeline, &mut ctx).await;
    assert_eq!(ctx, "before");
}

// Error message preservation
#[test]
fn test_error_message_happy_preserved() {
    let err = PipelineError::StepFailed("custom message".to_string());
    match err {
        PipelineError::StepFailed(msg) => assert_eq!(msg, "custom message"),
        _ => panic!("expected StepFailed"),
    }
}

#[test]
fn test_error_message_happy_empty() {
    let err = PipelineError::StepFailed("".to_string());
    match err {
        PipelineError::StepFailed(msg) => assert_eq!(msg, ""),
        _ => panic!("expected StepFailed"),
    }
}

#[test]
fn test_error_message_happy_long() {
    let long_msg = "x".repeat(1000);
    let err = PipelineError::StepFailed(long_msg.clone());
    match err {
        PipelineError::StepFailed(msg) => assert_eq!(msg, long_msg),
        _ => panic!("expected StepFailed"),
    }
}

// Error trait impl
#[test]
fn test_error_std_error_trait() {
    let err: Box<dyn std::error::Error> = Box::new(PipelineError::StepFailed("test".to_string()));
    assert!(!err.to_string().is_empty());
}

#[test]
fn test_error_display_trait() {
    let err = PipelineError::StepFailed("display test".to_string());
    let s = format!("{}", err);
    assert!(!s.is_empty());
}

#[test]
fn test_error_debug_trait() {
    let err = PipelineError::StepFailed("debug test".to_string());
    let s = format!("{:?}", err);
    assert!(!s.is_empty());
}
