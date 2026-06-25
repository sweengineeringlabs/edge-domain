//! @covers StepRegistry trait — happy/error/edge scenarios for register and build_pipeline.
//! Also covers the domain-registry backing store integration (InMemoryRegistry<dyn Step<Ctx>>).

use std::sync::Arc;

use edge_domain_pipeline::{
    PipelineDefinition, PipelineError, Step, StepRegistrySvc,
};
use edge_domain_registry::{Registry, RegistryBootstrap, StdRegistryFactory};

struct IncrementStep;

#[async_trait::async_trait]
impl Step<i32> for IncrementStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }
    fn name(&self) -> &str { "increment" }
}

struct FailStep;

#[async_trait::async_trait]
impl Step<i32> for FailStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("intentional".to_string()))
    }
    fn name(&self) -> &str { "fail" }
}

// ── register ─────────────────────────────────────────────────────────────────

/// @covers: register
#[tokio::test]
async fn test_register_happy_step_is_available_for_build() {
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("increment", Arc::new(IncrementStep));
    let def = PipelineDefinition { steps: vec!["increment".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("registered step must build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 1);
}

/// @covers: register
#[test]
fn test_register_error_unregistered_name_causes_unknown_step() {
    let registry = StepRegistrySvc::create::<i32>();
    let def = PipelineDefinition { steps: vec!["missing".to_owned()], ..Default::default() };
    match registry.build_pipeline(&def) {
        Err(PipelineError::UnknownStep(name)) => assert_eq!(name, "missing"),
        Err(e) => panic!("expected UnknownStep, got error: {:?}", e),
        Ok(_) => panic!("expected UnknownStep, got Ok"),
    }
}

/// @covers: register
#[tokio::test]
async fn test_register_edge_duplicate_name_overwrites() {
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("step", Arc::new(IncrementStep));
    registry.register("step", Arc::new(IncrementStep));
    let def = PipelineDefinition { steps: vec!["step".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("overwritten step must still build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 1);
}

// ── build_pipeline ────────────────────────────────────────────────────────────

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_happy_executes_steps_in_order() {
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("inc", Arc::new(IncrementStep));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "inc".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 3);
}

/// @covers: build_pipeline
#[test]
fn test_build_pipeline_error_unknown_step_name_first_miss_returned() {
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("known", Arc::new(IncrementStep));
    let def = PipelineDefinition {
        steps: vec!["known".to_owned(), "unknown".to_owned()],
        ..Default::default()
    };
    match registry.build_pipeline(&def) {
        Err(PipelineError::UnknownStep(name)) => assert_eq!(name, "unknown"),
        Err(e) => panic!("expected UnknownStep, got error: {:?}", e),
        Ok(_) => panic!("expected UnknownStep, got Ok"),
    }
}

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_edge_empty_steps_succeeds_immediately() {
    let registry = StepRegistrySvc::create::<i32>();
    let def = PipelineDefinition::default();
    let pipeline = registry.build_pipeline(&def).expect("empty pipeline is valid");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("empty pipeline succeeds");
    assert_eq!(ctx, 0);
}

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_happy_reuses_shared_step_instance() {
    let step: Arc<dyn Step<i32>> = Arc::new(IncrementStep);
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("inc", Arc::clone(&step));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    pipeline.run(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 2);
}

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_error_step_failure_propagates() {
    let mut registry = StepRegistrySvc::create::<i32>();
    registry.register("inc", Arc::new(IncrementStep));
    registry.register("fail", Arc::new(FailStep));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "fail".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}

// ── domain-registry backing store integration ─────────────────────────────────

/// @covers: StdRegistryFactory::in_memory
#[test]
fn test_backing_registry_happy_stores_step_by_name() {
    let reg = StdRegistryFactory::in_memory::<dyn Step<i32>>();
    reg.register("increment", Arc::new(IncrementStep));
    assert_eq!(
        reg.get("increment").map(|s| s.name().to_owned()),
        Some("increment".to_owned()),
        "registered step must be retrievable by name with correct identity"
    );
}

/// @covers: StdRegistryFactory::in_memory
#[test]
fn test_backing_registry_error_absent_name_returns_none() {
    let reg = StdRegistryFactory::in_memory::<dyn Step<i32>>();
    assert!(reg.get("absent").is_none(), "unregistered name must return None");
    assert!(reg.is_empty(), "factory-created registry must start empty");
}

/// @covers: StdRegistryFactory::in_memory
#[test]
fn test_backing_registry_edge_duplicate_register_overwrites() {
    let reg = StdRegistryFactory::in_memory::<dyn Step<i32>>();
    reg.register("step", Arc::new(IncrementStep));
    reg.register("step", Arc::new(FailStep));
    assert_eq!(reg.len(), 1, "duplicate registrations must not increase count");
    assert_eq!(
        reg.get("step").map(|s| s.name().to_owned()),
        Some("fail".to_owned()),
        "last registered step must win"
    );
}
