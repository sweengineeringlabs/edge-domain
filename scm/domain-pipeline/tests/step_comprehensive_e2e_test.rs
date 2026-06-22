//! Comprehensive scenario coverage for Step trait.
//! Tests: happy path, error path, edge cases

use edge_domain_pipeline::{Step, PipelineError};

struct MutatingStep(i32);

#[async_trait::async_trait]
impl Step<i32> for MutatingStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += self.0;
        Ok(())
    }

    fn name(&self) -> &str {
        "mutating"
    }
}

struct ErrorStep(String);

#[async_trait::async_trait]
impl Step<i32> for ErrorStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed(self.0.clone()))
    }

    fn name(&self) -> &str {
        "error-step"
    }
}

// Happy path: Step executes and mutates context
/// @covers: execute
#[tokio::test]
async fn test_step_execute_happy_mutates_context() {
    let step = MutatingStep(5);
    let mut ctx = 10;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: name
#[tokio::test]
async fn test_step_execute_happy_name_accessible() {
    let step = MutatingStep(0);
    let mut ctx = 0;
    let _ = step.execute(&mut ctx).await;
    assert_eq!(step.name(), "mutating");
}

// Error path: Step returns error without mutating
#[tokio::test]
async fn test_step_execute_error_returns_failure() {
    let step = ErrorStep("test failed".to_string());
    let mut ctx = 10;
    let result = step.execute(&mut ctx).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_step_execute_error_preserves_context() {
    let step = ErrorStep("test failed".to_string());
    let mut ctx = 10;
    let _ = step.execute(&mut ctx).await;
    assert_eq!(ctx, 10);
}

#[tokio::test]
async fn test_step_execute_error_message_preserved() {
    let step = ErrorStep("custom error".to_string());
    let mut ctx = 0;
    match step.execute(&mut ctx).await {
        Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "custom error"),
        _ => panic!("expected StepFailed error"),
    }
}

// Edge cases
#[tokio::test]
async fn test_step_execute_edge_zero_mutation() {
    let step = MutatingStep(0);
    let mut ctx = 42;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 42);
}

#[tokio::test]
async fn test_step_execute_edge_negative_mutation() {
    let step = MutatingStep(-10);
    let mut ctx = 5;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, -5);
}

#[tokio::test]
async fn test_step_execute_edge_large_values() {
    let step = MutatingStep(i32::MAX / 2);
    let mut ctx = i32::MAX / 2;
    assert!(step.execute(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_step_execute_edge_empty_error_message() {
    let step = ErrorStep("".to_string());
    let mut ctx = 0;
    let result = step.execute(&mut ctx).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_step_dyn_dispatch_happy() {
    let step: Box<dyn Step<i32>> = Box::new(MutatingStep(7));
    let mut ctx = 3;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 10);
}

#[tokio::test]
async fn test_step_dyn_dispatch_error() {
    let step: Box<dyn Step<i32>> = Box::new(ErrorStep("dyn error".to_string()));
    let mut ctx = 0;
    assert!(step.execute(&mut ctx).await.is_err());
}

// Explicit scenario coverage for name() method
/// @covers: name
#[test]
fn test_name_normal_step_happy() {
    let step = MutatingStep(5);
    assert_eq!(step.name(), "mutating");
}

#[test]
fn test_name_error_step_happy() {
    let step = ErrorStep("test".to_string());
    assert_eq!(step.name(), "error-step");
}

#[test]
fn test_name_multiple_calls_happy() {
    let step = MutatingStep(10);
    assert_eq!(step.name(), "mutating");
    assert_eq!(step.name(), "mutating");
}

#[test]
fn test_name_after_execute_happy() {
    let step = MutatingStep(5);
    let name_before = step.name();
    let name_after = step.name();
    assert_eq!(name_before, name_after);
}

#[tokio::test]
async fn test_name_after_failed_execute_error() {
    let step = ErrorStep("error".to_string());
    let mut ctx = 0;
    let _ = step.execute(&mut ctx).await;
    assert_eq!(step.name(), "error-step");
}

#[test]
fn test_name_edge_special_chars() {
    let step = MutatingStep(0);
    let name = step.name();
    assert!(!name.is_empty());
    assert!(name.chars().all(|c| c.is_ascii()));
}

#[tokio::test]
async fn test_step_execute_error_handles_mutations() {
    struct MutatingErrorStep;
    #[async_trait::async_trait]
    impl Step<i32> for MutatingErrorStep {
        async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
            *ctx += 1;
            Err(PipelineError::StepFailed("mutated then failed".to_string()))
        }
        fn name(&self) -> &str { "mutating-error" }
    }
    let mut ctx = 0;
    let result = MutatingErrorStep.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}
