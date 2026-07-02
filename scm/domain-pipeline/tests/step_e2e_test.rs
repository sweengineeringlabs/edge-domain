//! End-to-end coverage for the [`Step`] trait's public method surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Step, StepFailureRequest, StepNameRequest, StepNameResponse,
};

struct AddStep(i32);

#[async_trait::async_trait]
impl Step for AddStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        *req.ctx += self.0;
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: "add-step".to_string(),
        })
    }
}

struct DefaultNameStep;

#[async_trait::async_trait]
impl Step for DefaultNameStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        *req.ctx = 0;
        Ok(())
    }
}

/// @covers: execute
#[tokio::test]
async fn test_execute_happy_mutates_context() {
    let step = AddStep(5);
    let mut ctx = 10;
    step.execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("execute must succeed");
    assert_eq!(ctx, 15);
}

/// @covers: execute
#[tokio::test]
async fn test_execute_error_returns_typed_cause() {
    struct FailStep;
    #[async_trait::async_trait]
    impl Step for FailStep {
        type Ctx = i32;
        type ExecutionError = String;

        async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
            Err("execution failed".to_string())
        }
    }
    let step = FailStep;
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert_eq!(result, Err("execution failed".to_string()));
}

/// @covers: name
#[test]
fn test_name_happy_returns_overridden_name() {
    let step = AddStep(1);
    let response = step.name(StepNameRequest).expect("name must succeed");
    assert_eq!(response.name, "add-step");
}

/// @covers: name
#[test]
fn test_name_edge_default_body_returns_type_name() {
    let step = DefaultNameStep;
    let response = step.name(StepNameRequest).expect("name must succeed");
    assert!(
        response.name.contains("DefaultNameStep"),
        "default name body must fall back to the concrete type name, got {}",
        response.name
    );
}

/// @covers: fail_with
#[test]
fn test_fail_with_happy_wraps_step_name_and_cause() {
    let step = AddStep(1);
    let response = step
        .fail_with(StepFailureRequest {
            step_name: "add-step".to_string(),
            cause: "overflow".to_string(),
        })
        .expect("fail_with must succeed");
    assert_eq!(response.error.step_name, "add-step");
    assert_eq!(response.error.cause, "overflow");
}

/// @covers: fail_with
#[test]
fn test_fail_with_edge_empty_step_name_still_preserves_cause() {
    let step = AddStep(1);
    let response = step
        .fail_with(StepFailureRequest {
            step_name: String::new(),
            cause: "cause".to_string(),
        })
        .expect("fail_with must succeed");
    assert_eq!(response.error.step_name, "");
    assert_eq!(response.error.cause, "cause");
}
