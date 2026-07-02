//! @covers error handling and edge cases
//! Error scenario tests for PipelineError and error handling.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fmt;
use std::sync::Arc;

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
    StepError, StepNameRequest, StepNameResponse,
};

struct ErrorWithContext(String);

#[async_trait::async_trait]
impl Step<String, String> for ErrorWithContext {
    async fn execute(&self, req: ContextMutationRequest<'_, String>) -> Result<(), String> {
        req.ctx.push_str(&self.0);
        Err("step error".to_string())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "error-step".to_string(),
        })
    }
}

// PipelineError variants — Display
/// @covers: general
#[test]
fn test_error_step_failed_happy_edge() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "x".to_string(),
        cause: "test error".to_string(),
    });
    let msg = format!("{}", err);
    assert!(msg.contains("test error"));
}

/// @covers: general
#[test]
fn test_error_step_timeout_happy_edge() {
    let err: PipelineError<String> = PipelineError::StepTimeout {
        step_name: "x".to_string(),
    };
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}

/// @covers: general
#[test]
fn test_error_config_error_happy_edge() {
    let err: PipelineError<String> = PipelineError::ConfigError("config issue".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("config issue"));
}

// Error propagation
/// @covers: general
#[tokio::test]
async fn test_error_propagation_stops_pipeline_happy() {
    let steps: Vec<Arc<dyn Step<String, String>>> =
        vec![Arc::new(ErrorWithContext("partial".to_string()))];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = String::new();
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, "partial");
}

/// @covers: general
#[tokio::test]
async fn test_error_context_mutation_before_error_happy() {
    let steps: Vec<Arc<dyn Step<String, String>>> =
        vec![Arc::new(ErrorWithContext("before".to_string()))];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = String::new();
    let _ = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert_eq!(ctx, "before");
}

// Error message preservation — cause is in StepError.cause
/// @covers: general
#[test]
fn test_error_message_preserved_happy() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "custom message".to_string(),
    });
    match err {
        PipelineError::StepFailed(e) => assert_eq!(e.cause, "custom message"),
        _ => panic!("expected StepFailed"),
    }
}

/// @covers: general
#[test]
fn test_error_message_empty_happy() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "".to_string(),
    });
    match err {
        PipelineError::StepFailed(e) => assert_eq!(e.cause, ""),
        _ => panic!("expected StepFailed"),
    }
}

/// @covers: general
#[test]
fn test_error_message_long_happy() {
    let long_msg = "x".repeat(1000);
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: long_msg.clone(),
    });
    match err {
        PipelineError::StepFailed(e) => assert_eq!(e.cause, long_msg),
        _ => panic!("expected StepFailed"),
    }
}

// std::error::Error trait impl — requires E: std::error::Error + Send + 'static
// String doesn't impl std::error::Error, so use a newtype wrapper for this test.
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
fn test_error_std_error_trait_happy() {
    let err: Box<dyn std::error::Error> =
        Box::new(PipelineError::<AnyError>::StepFailed(StepError {
            step_name: "test".to_string(),
            cause: AnyError("test".to_string()),
        }));
    assert!(!err.to_string().is_empty());
}

/// @covers: general
#[test]
fn test_error_display_trait_happy() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "display test".to_string(),
    });
    let s = format!("{}", err);
    assert!(!s.is_empty());
}

/// @covers: general
#[test]
fn test_error_debug_trait_happy() {
    let err: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "debug test".to_string(),
    });
    let s = format!("{:?}", err);
    assert!(!s.is_empty());
}

// Edge case: multiple error types in sequence
/// @covers: general
#[test]
fn test_error_multiple_error_types_edge() {
    let e1: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "step".to_string(),
    });
    let e2: PipelineError<String> = PipelineError::StepTimeout {
        step_name: "s".to_string(),
    };
    let e3: PipelineError<String> = PipelineError::ConfigError("config".to_string());

    match e1 {
        PipelineError::StepFailed(_) => {}
        _ => panic!("expected StepFailed"),
    }
    match e2 {
        PipelineError::StepTimeout { .. } => {}
        _ => panic!("expected StepTimeout"),
    }
    match e3 {
        PipelineError::ConfigError(_) => {}
        _ => panic!("expected ConfigError"),
    }
}

#[test]
fn test_error_variants_distinct_edge() {
    let e1: PipelineError<String> = PipelineError::StepFailed(StepError {
        step_name: "s".to_string(),
        cause: "msg1".to_string(),
    });
    let e2: PipelineError<String> = PipelineError::StepTimeout {
        step_name: "s".to_string(),
    };
    let e3: PipelineError<String> = PipelineError::ConfigError("msg3".to_string());
    assert!(matches!(e1, PipelineError::StepFailed(_)));
    assert!(matches!(e2, PipelineError::StepTimeout { .. }));
    assert!(matches!(e3, PipelineError::ConfigError(_)));
}

// pipeline error wraps step cause correctly
#[tokio::test]
async fn test_pipeline_run_wraps_step_error_with_step_name() {
    let steps: Vec<Arc<dyn Step<String, String>>> =
        vec![Arc::new(ErrorWithContext("x".to_string()))];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = String::new();
    match pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await {
        Err(PipelineError::StepFailed(e)) => {
            assert_eq!(e.step_name, "error-step");
            assert_eq!(e.cause, "step error");
        }
        other => panic!("expected StepFailed, got {:?}", other),
    }
}
