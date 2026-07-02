//! Integration tests for step and step-registry service facades.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineAssemblyRequest, PipelineBuilder, PipelineDefinition,
    PipelineError, PipelineSvc, Step, StepNameRequest, StepNameResponse, StepRegistrationRequest,
    StepRegistry, StepRegistrySvc, StepSvc, STEP_REGISTRY_SVC, STEP_SVC,
};
use std::sync::Arc;

struct TestStep(i32);

#[async_trait::async_trait]
impl Step for TestStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        *req.ctx += self.0;
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "test".to_string(),
        })
    }
}

struct FailStep;

#[async_trait::async_trait]
impl Step for FailStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Err("boom".to_string())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "fail".to_string(),
        })
    }
}

// Test STEP_SVC constant
/// @covers: general
#[test]
fn test_step_svc_constant() {
    assert_eq!(STEP_SVC, "step");
}

// Test Step trait usage through factory
/// @covers: general
#[tokio::test]
async fn test_step_svc_step_trait_happy_execute() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::new(TestStep(5));
    let mut ctx = 10;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: general
#[test]
fn test_step_svc_step_trait_happy_name() {
    let step: Box<dyn Step<Ctx = i32, ExecutionError = String>> = Box::new(TestStep(0));
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "test"
    );
}

/// @covers: general
#[tokio::test]
async fn test_step_svc_step_trait_edge_different_values() {
    let step1: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::new(TestStep(10));
    let step2: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::new(TestStep(-5));

    let mut ctx1 = 0;
    let mut ctx2 = 0;

    assert!(step1
        .execute(ContextMutationRequest { ctx: &mut ctx1 })
        .await
        .is_ok());
    assert_eq!(ctx1, 10);

    assert!(step2
        .execute(ContextMutationRequest { ctx: &mut ctx2 })
        .await
        .is_ok());
    assert_eq!(ctx2, -5);
}

// ── StepSvc::noop / noop_shared ───────────────────────────────────────────────
// `noop`/`noop_shared` themselves have no failure path (no Result in either
// signature), but the step they construct must not interfere with error
// propagation when used alongside a failing step in a real pipeline — that is
// the meaningful "_error" scenario for a no-op building block, and it can
// genuinely fail (e.g. if the no-op step swallowed or misrouted a downstream
// error).

/// @covers: StepSvc::noop
#[tokio::test]
async fn test_noop_does_not_suppress_downstream_step_error() {
    let noop: Box<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop();
    let noop: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::from(noop);
    let pipeline: Box<dyn Pipeline<Ctx = i32, E = String, Request = i32, Response = i32>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(noop).with(FailStep));
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(
        result.is_err(),
        "a noop step preceding a failing step must not suppress the downstream error"
    );
    assert_eq!(
        ctx, 0,
        "noop must not have mutated context before the failure"
    );
}

/// @covers: StepSvc::noop
#[tokio::test]
async fn test_noop_leaves_context_unchanged_happy() {
    let step: Box<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop();
    let mut ctx = 42;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 42, "noop must not mutate the context");
}

/// @covers: StepSvc::noop
#[tokio::test]
async fn test_noop_different_ctx_type_edge() {
    let step: Box<dyn Step<Ctx = String, ExecutionError = String>> = StepSvc::noop();
    let mut ctx = "unchanged".to_string();
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, "unchanged", "noop must be context-type agnostic");
}

/// @covers: StepSvc::noop
#[test]
fn test_noop_reports_default_step_name_edge() {
    let step: Box<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop();
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "default-step"
    );
}

/// @covers: StepSvc::noop_shared
#[tokio::test]
async fn test_noop_shared_does_not_suppress_downstream_step_error() {
    let shared: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop_shared();
    let pipeline: Box<dyn Pipeline<Ctx = i32, E = String, Request = i32, Response = i32>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with_shared(Arc::clone(&shared))
                .with(FailStep),
        );
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(
        result.is_err(),
        "a shared noop step preceding a failing step must not suppress the downstream error"
    );
    assert_eq!(
        Arc::strong_count(&shared),
        2,
        "the pipeline must hold its own clone of the shared step, not consume the original"
    );
}

/// @covers: StepSvc::noop_shared
#[tokio::test]
async fn test_noop_shared_leaves_context_unchanged_happy() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop_shared();
    let mut ctx = 7;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 7, "noop_shared must not mutate the context");
}

/// @covers: StepSvc::noop_shared
#[test]
fn test_noop_shared_clone_increments_strong_count_edge() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop_shared();
    let before = Arc::strong_count(&step);
    let cloned = Arc::clone(&step);
    assert_eq!(
        Arc::strong_count(&cloned),
        before + 1,
        "noop_shared must return a genuinely shareable Arc"
    );
}

/// @covers: StepSvc::noop_shared
#[test]
fn test_noop_shared_reports_default_step_name_edge() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = StepSvc::noop_shared();
    assert_eq!(
        step.name(StepNameRequest).expect("must succeed").name,
        "default-step"
    );
}

// ── StepRegistry service facade ───────────────────────────────────────────────

/// @covers: STEP_REGISTRY_SVC
#[test]
fn test_step_registry_svc_constant() {
    assert_eq!(STEP_REGISTRY_SVC, "step_registry");
    assert_ne!(STEP_REGISTRY_SVC, STEP_SVC);
}

/// @covers: StepRegistry::register, StepRegistry::build_pipeline
#[tokio::test]
async fn test_step_registry_svc_happy_register_and_execute() {
    let mut registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    registry
        .register(StepRegistrationRequest {
            name: "add5".to_string(),
            step: Arc::new(TestStep(5)),
        })
        .expect("register must succeed");
    let def = PipelineDefinition {
        steps: vec!["add5".to_owned()],
        ..Default::default()
    };
    let pipeline = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(def),
        })
        .expect("registered step must build")
        .pipeline;
    let mut ctx = 0i32;
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("pipeline must execute");
    assert_eq!(ctx, 5);
}

/// @covers: StepRegistry::build_pipeline
#[test]
fn test_step_registry_svc_error_unknown_step_rejected() {
    let registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    let def = PipelineDefinition {
        steps: vec!["ghost".to_owned()],
        ..Default::default()
    };
    let result = registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(def),
    });
    assert!(result.is_err(), "unregistered step name must be rejected");
}

/// @covers: StepRegistry::register
#[tokio::test]
async fn test_step_registry_svc_edge_empty_definition_succeeds() {
    let registry: Box<dyn StepRegistry<Ctx = i32, E = String>> =
        StepRegistrySvc::create::<i32, String>();
    let def = PipelineDefinition::default();
    let pipeline = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(def),
        })
        .expect("empty definition must build")
        .pipeline;
    let mut ctx = 0i32;
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("empty pipeline must execute");
    assert_eq!(ctx, 0);
}

// Suppress unused import — PipelineError is used via StepRegistry's return type
#[allow(dead_code)]
fn _use_pipeline_error(_: PipelineError<String>) {}
