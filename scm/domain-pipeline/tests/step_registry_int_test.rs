//! @covers StepRegistry trait — happy/error/edge scenarios for register and build_pipeline.

use std::sync::Arc;

use edge_domain_pipeline::{
    create_step_registry, PipelineDefinition, PipelineError, Step,
};

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
    let mut registry = create_step_registry::<i32>();
    registry.register("increment", Arc::new(IncrementStep));
    let def = PipelineDefinition { steps: vec!["increment".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("registered step must build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 1);
}

/// @covers: register
#[test]
fn test_register_error_unregistered_name_causes_unknown_step() {
    let registry = create_step_registry::<i32>();
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
    let mut registry = create_step_registry::<i32>();
    registry.register("step", Arc::new(IncrementStep));
    registry.register("step", Arc::new(IncrementStep)); // second registration replaces first
    let def = PipelineDefinition { steps: vec!["step".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("overwritten step must still build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 1); // exactly one step registered; duplicate didn't double it
}

// ── build_pipeline ────────────────────────────────────────────────────────────

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_happy_executes_steps_in_order() {
    let mut registry = create_step_registry::<i32>();
    registry.register("inc", Arc::new(IncrementStep));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "inc".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 3);
}

/// @covers: build_pipeline
#[test]
fn test_build_pipeline_error_unknown_step_name_first_miss_returned() {
    let mut registry = create_step_registry::<i32>();
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
    let registry = create_step_registry::<i32>();
    let def = PipelineDefinition::default(); // steps: vec![]
    let pipeline = registry.build_pipeline(&def).expect("empty pipeline is valid");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("empty pipeline succeeds");
    assert_eq!(ctx, 0);
}

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_happy_reuses_shared_step_instance() {
    let step: Arc<dyn Step<i32>> = Arc::new(IncrementStep);
    let mut registry = create_step_registry::<i32>();
    registry.register("inc", Arc::clone(&step));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("should succeed");
    assert_eq!(ctx, 2);
}

/// @covers: build_pipeline
#[tokio::test]
async fn test_build_pipeline_error_step_failure_propagates() {
    let mut registry = create_step_registry::<i32>();
    registry.register("inc", Arc::new(IncrementStep));
    registry.register("fail", Arc::new(FailStep));
    let def = PipelineDefinition {
        steps: vec!["inc".to_owned(), "fail".to_owned(), "inc".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("should build");
    let mut ctx = 0i32;
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1); // first increment ran; fail aborted
}
