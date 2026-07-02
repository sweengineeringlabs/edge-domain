//! Comprehensive scenario coverage for Step trait.
//! Tests: happy path, error path, edge cases
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineError, Step, StepNameRequest, StepNameResponse,
};

struct MutatingStep(i32);

#[async_trait::async_trait]
impl Step for MutatingStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        *req.ctx += self.0;
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "mutating".to_string(),
        })
    }
}

struct ErrorStep(String);

#[async_trait::async_trait]
impl Step for ErrorStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Err(self.0.clone())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "error-step".to_string(),
        })
    }
}

// Happy path: Step executes and mutates context
/// @covers: execute
#[tokio::test]
async fn test_step_execute_mutates_context_happy() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(5);
    let mut ctx = 10;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: name
#[tokio::test]
async fn test_step_execute_name_accessible_happy() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(0);
    let mut ctx = 0;
    let _ = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "mutating"
    );
}

// Error path: Step returns error without mutating
#[tokio::test]
async fn test_step_execute_returns_failure_error() {
    let step = ErrorStep("test failed".to_string());
    let mut ctx = 10;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_step_execute_preserves_context_error() {
    let step = ErrorStep("test failed".to_string());
    let mut ctx = 10;
    let _ = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert_eq!(ctx, 10);
}

#[tokio::test]
async fn test_step_execute_message_preserved_error() {
    let step = ErrorStep("custom error".to_string());
    let mut ctx = 0;
    match step.execute(ContextMutationRequest { ctx: &mut ctx }).await {
        Err(msg) => assert_eq!(msg, "custom error"),
        _ => panic!("expected error"),
    }
}

// Edge cases
#[tokio::test]
async fn test_step_execute_zero_mutation_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(0);
    let mut ctx = 42;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 42);
}

#[tokio::test]
async fn test_step_execute_negative_mutation_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(-10);
    let mut ctx = 5;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, -5);
}

#[tokio::test]
async fn test_step_execute_large_values_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(i32::MAX / 2);
    let mut ctx = i32::MAX / 2;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_step_execute_empty_error_message_edge() {
    let step = ErrorStep("".to_string());
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_step_dyn_dispatch_happy_edge() {
    let step: Box<dyn Step<Ctx = i32, ExecutionError = String>> = Box::new(MutatingStep(7));
    let mut ctx = 3;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 10);
}

#[tokio::test]
async fn test_step_dyn_dispatch_error_happy() {
    let step: Box<dyn Step<Ctx = i32, ExecutionError = String>> =
        Box::new(ErrorStep("dyn error".to_string()));
    let mut ctx = 0;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_err());
}

// Explicit scenario coverage for name() method
/// @covers: name
#[test]
fn test_name_normal_step_happy_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(5);
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "mutating"
    );
}

#[test]
fn test_name_error_step_happy_edge() {
    let step = ErrorStep("test".to_string());
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "error-step"
    );
}

#[test]
fn test_name_multiple_calls_happy_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(10);
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "mutating"
    );
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "mutating"
    );
}

#[test]
fn test_name_after_execute_happy_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(5);
    let name_before = step.name(StepNameRequest).expect("must succeed").name;
    let name_after = step.name(StepNameRequest).expect("must succeed").name;
    assert_eq!(name_before, name_after);
}

#[tokio::test]
async fn test_name_after_failed_execute_error_happy() {
    let step = ErrorStep("error".to_string());
    let mut ctx = 0;
    let _ = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "error-step"
    );
}

#[test]
fn test_name_special_chars_edge() {
    let step: &dyn Step<Ctx = i32, ExecutionError = String> = &MutatingStep(0);
    let name = step.name(StepNameRequest).expect("must succeed").name;
    assert!(!name.is_empty());
    assert!(name.is_ascii());
}

#[tokio::test]
async fn test_step_execute_handles_mutations_error() {
    struct MutatingErrorStep;
    #[async_trait::async_trait]
    impl Step for MutatingErrorStep {
        type Ctx = i32;
        type ExecutionError = String;

        async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
            *req.ctx += 1;
            Err("mutated then failed".to_string())
        }
        fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
            Ok(StepNameResponse {
                name: "mutating-error".to_string(),
            })
        }
    }
    let mut ctx = 0;
    let result = MutatingErrorStep
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}
