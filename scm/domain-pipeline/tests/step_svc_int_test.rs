//! Integration tests for step and step-registry service facades.

use edge_domain_pipeline::{
    PipelineDefinition, PipelineError, Step, StepRegistry, StepRegistrySvc,
    STEP_REGISTRY_SVC, STEP_SVC,
};
use std::sync::Arc;

struct TestStep(i32);

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for TestStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), E> {
        *ctx += self.0;
        Ok(())
    }

    fn name(&self) -> &str {
        "test"
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
    let step: Arc<dyn Step<i32, String>> = Arc::new(TestStep(5));
    let mut ctx = 10;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: general
#[test]
fn test_step_svc_step_trait_happy_name() {
    let step: Box<dyn Step<i32, String>> = Box::new(TestStep(0));
    assert_eq!(step.name(), "test");
}

/// @covers: general
#[tokio::test]
async fn test_step_svc_step_trait_edge_different_values() {
    let step1: Arc<dyn Step<i32, String>> = Arc::new(TestStep(10));
    let step2: Arc<dyn Step<i32, String>> = Arc::new(TestStep(-5));

    let mut ctx1 = 0;
    let mut ctx2 = 0;

    assert!(step1.execute(&mut ctx1).await.is_ok());
    assert_eq!(ctx1, 10);

    assert!(step2.execute(&mut ctx2).await.is_ok());
    assert_eq!(ctx2, -5);
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
    let mut registry: Box<dyn StepRegistry<Ctx = i32, E = String>> = StepRegistrySvc::create::<i32, String>();
    registry.register("add5", Arc::new(TestStep(5)));
    let def = PipelineDefinition { steps: vec!["add5".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("registered step must build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 5);
}

/// @covers: StepRegistry::build_pipeline
#[test]
fn test_step_registry_svc_error_unknown_step_rejected() {
    let registry: Box<dyn StepRegistry<Ctx = i32, E = String>> = StepRegistrySvc::create::<i32, String>();
    let def = PipelineDefinition { steps: vec!["ghost".to_owned()], ..Default::default() };
    let result = registry.build_pipeline(&def);
    assert!(result.is_err(), "unregistered step name must be rejected");
}

/// @covers: StepRegistry::register
#[tokio::test]
async fn test_step_registry_svc_edge_empty_definition_succeeds() {
    let registry: Box<dyn StepRegistry<Ctx = i32, E = String>> = StepRegistrySvc::create::<i32, String>();
    let def = PipelineDefinition::default();
    let pipeline = registry.build_pipeline(&def).expect("empty definition must build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("empty pipeline must execute");
    assert_eq!(ctx, 0);
}

// Suppress unused import — PipelineError is used via StepRegistry's return type
#[allow(dead_code)]
fn _use_pipeline_error(_: PipelineError<String>) {}
