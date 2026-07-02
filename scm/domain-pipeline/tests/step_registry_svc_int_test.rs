//! Integration tests for the step-registry service facade (STEP_REGISTRY_SVC + StepRegistry).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineAssemblyRequest, PipelineDefinition, PipelineError, Step,
    StepFailureRequest, StepNameRequest, StepNameResponse, StepRegistrationRequest, StepRegistry,
    StepRegistrySvc, STEP_REGISTRY_SVC, STEP_SVC,
};

struct AddStep(i32);

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for AddStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        *req.ctx += self.0;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "add".to_string(),
        })
    }
}

// ── STEP_REGISTRY_SVC constant ────────────────────────────────────────────────

/// @covers: STEP_REGISTRY_SVC
#[test]
fn test_step_registry_svc_constant_happy_value() {
    assert_eq!(STEP_REGISTRY_SVC, "step_registry");
}

/// @covers: STEP_REGISTRY_SVC
#[test]
fn test_step_registry_svc_constant_error_differs_from_step_svc() {
    assert_ne!(STEP_REGISTRY_SVC, STEP_SVC);
}

/// @covers: STEP_REGISTRY_SVC
#[test]
fn test_step_registry_svc_constant_edge_is_snake_case() {
    assert_eq!(STEP_REGISTRY_SVC, STEP_REGISTRY_SVC.to_lowercase());
    assert!(!STEP_REGISTRY_SVC.is_empty());
}

// ── StepRegistry trait ────────────────────────────────────────────────────────

/// @covers: StepRegistry
#[tokio::test]
async fn test_step_registry_svc_happy_multi_step_pipeline_executes_all() {
    let mut registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    registry
        .register(StepRegistrationRequest {
            name: "add10".to_string(),
            step: Arc::new(AddStep(10)),
        })
        .expect("must succeed");
    registry
        .register(StepRegistrationRequest {
            name: "add5".to_string(),
            step: Arc::new(AddStep(5)),
        })
        .expect("must succeed");
    let def = PipelineDefinition {
        steps: vec!["add10".to_owned(), "add5".to_owned()],
        ..Default::default()
    };
    let pipeline = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(def),
        })
        .expect("pipeline must build")
        .pipeline;
    let mut ctx = 0i32;
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("pipeline must execute");
    assert_eq!(ctx, 15);
}

/// @covers: StepRegistry
#[test]
fn test_step_registry_svc_error_build_fails_on_first_unknown() {
    let mut registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    registry
        .register(StepRegistrationRequest {
            name: "known".to_string(),
            step: Arc::new(AddStep(1)),
        })
        .expect("must succeed");
    let def = PipelineDefinition {
        steps: vec!["known".to_owned(), "absent".to_owned()],
        ..Default::default()
    };
    match registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(def),
    }) {
        Err(PipelineError::UnknownStep(name)) => assert_eq!(name, "absent"),
        Err(e) => panic!("expected UnknownStep(absent), got error: {:?}", e),
        Ok(_) => panic!("expected UnknownStep, got Ok"),
    }
}

// ── step_error_for ────────────────────────────────────────────────────────────

/// @covers: step_error_for
#[test]
fn test_step_error_for_happy_wraps_name_and_cause() {
    let registry = StepRegistrySvc::create::<i32, String>();
    let err = registry
        .step_error_for(StepFailureRequest {
            step_name: "my-step".to_string(),
            cause: "some error".to_string(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(err.step_name, "my-step");
    assert_eq!(err.cause, "some error");
}

/// @covers: step_error_for
#[test]
fn test_step_error_for_error_empty_step_name_still_sets_cause() {
    let registry = StepRegistrySvc::create::<i32, String>();
    let err = registry
        .step_error_for(StepFailureRequest {
            step_name: "".to_string(),
            cause: "cause".to_string(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(err.step_name, "");
    assert_eq!(err.cause, "cause");
}

/// @covers: step_error_for
#[test]
fn test_step_error_for_edge_whitespace_step_name() {
    let registry = StepRegistrySvc::create::<i32, String>();
    let err = registry
        .step_error_for(StepFailureRequest {
            step_name: "  spaces  ".to_string(),
            cause: "cause".to_string(),
        })
        .expect("must succeed")
        .error;
    assert_eq!(
        err.step_name, "  spaces  ",
        "whitespace in step name must be preserved verbatim"
    );
    assert_eq!(err.cause, "cause");
}

/// @covers: StepRegistry
#[tokio::test]
async fn test_step_registry_svc_edge_step_registered_multiple_times_only_last_counts() {
    let mut registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    registry
        .register(StepRegistrationRequest {
            name: "s".to_string(),
            step: Arc::new(AddStep(100)),
        })
        .expect("must succeed");
    registry
        .register(StepRegistrationRequest {
            name: "s".to_string(),
            step: Arc::new(AddStep(1)),
        })
        .expect("must succeed");
    let def = PipelineDefinition {
        steps: vec!["s".to_owned()],
        ..Default::default()
    };
    let pipeline = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(def),
        })
        .expect("must build")
        .pipeline;
    let mut ctx = 0i32;
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("must execute");
    assert_eq!(ctx, 1);
}
