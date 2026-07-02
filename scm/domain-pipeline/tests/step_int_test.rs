//! Integration tests for the [`Step`] trait contract.
//!
//! @covers Step
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineError, Step, StepFailureRequest, StepNameRequest,
    StepNameResponse,
};

struct CountingStep {
    name: String,
}

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for CountingStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        *req.ctx += 1;
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: self.name.clone(),
        })
    }
}

/// @covers: general
#[tokio::test]
async fn trait_step_executes_and_mutates_context() {
    let step = CountingStep {
        name: "increment".to_string(),
    };
    let step_dyn: &dyn Step<i32, String> = &step;
    let mut ctx = 5;
    assert!(step_dyn
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 6);
}

/// @covers: general
#[tokio::test]
async fn trait_step_name_is_accessible() {
    let step = CountingStep {
        name: "my-step".to_string(),
    };
    let step_ref: &dyn Step<i32, String> = &step;
    assert_eq!(
        step_ref.name(StepNameRequest).expect("must succeed").name,
        "my-step"
    );
}

/// @covers: general
#[tokio::test]
async fn trait_step_error_halts_mutation() {
    struct FailingStep;

    #[async_trait::async_trait]
    impl Step<String, String> for FailingStep {
        async fn execute(&self, _req: ContextMutationRequest<'_, String>) -> Result<(), String> {
            Err("forced failure".to_string())
        }

        fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
            Ok(StepNameResponse {
                name: "failing-step".to_string(),
            })
        }
    }

    let step = FailingStep;
    let mut ctx = "initial".to_string();
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, "initial");
}

/// @covers: general
#[tokio::test]
async fn trait_step_dyn_dispatch_works() {
    let step1: Box<dyn Step<i32, String>> = Box::new(CountingStep {
        name: "step1".to_string(),
    });
    let step2: Box<dyn Step<i32, String>> = Box::new(CountingStep {
        name: "step2".to_string(),
    });

    let mut ctx = 0;
    assert!(step1
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 1);

    assert!(step2
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 2);
}

// ── fail_with ─────────────────────────────────────────────────────────────────

/// @covers: fail_with
#[test]
fn test_fail_with_happy_wraps_cause_with_step_name() {
    let step = CountingStep {
        name: "counter".to_string(),
    };
    let err = step
        .fail_with(StepFailureRequest {
            step_name: "counter".to_string(),
            cause: "injected error".to_string(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(
        err.step_name, "counter",
        "step name must be preserved in the error"
    );
    assert_eq!(
        err.cause, "injected error",
        "cause must be preserved verbatim"
    );
}

/// @covers: fail_with
#[test]
fn test_fail_with_error_cause_preserved_verbatim() {
    let step = CountingStep {
        name: "writer".to_string(),
    };
    let cause = "write failed: disk full".to_string();
    let err = step
        .fail_with(StepFailureRequest {
            step_name: "writer".to_string(),
            cause: cause.clone(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(
        err.cause, cause,
        "multi-word cause must be preserved exactly"
    );
    assert_eq!(err.step_name, "writer");
}

/// @covers: fail_with
#[test]
fn test_fail_with_edge_empty_step_name() {
    let step = CountingStep {
        name: String::new(),
    };
    let err = step
        .fail_with(StepFailureRequest {
            step_name: String::new(),
            cause: "cause".to_string(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(
        err.step_name, "",
        "empty step name must survive into the error"
    );
    assert_eq!(err.cause, "cause");
}
