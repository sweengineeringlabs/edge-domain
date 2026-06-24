//! Integration tests for the step-registry service facade (STEP_REGISTRY_SVC + StepRegistry).

use std::sync::Arc;

use edge_domain_pipeline::{
    create_step_registry, PipelineDefinition, PipelineError, Step, StepRegistry,
    STEP_REGISTRY_SVC, STEP_SVC,
};

struct AddStep(i32);

#[async_trait::async_trait]
impl Step<i32> for AddStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += self.0;
        Ok(())
    }
    fn name(&self) -> &str { "add" }
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
    let mut registry: Box<dyn StepRegistry<i32>> = create_step_registry();
    registry.register("add10", Arc::new(AddStep(10)));
    registry.register("add5", Arc::new(AddStep(5)));
    let def = PipelineDefinition {
        steps: vec!["add10".to_owned(), "add5".to_owned()],
        ..Default::default()
    };
    let pipeline = registry.build_pipeline(&def).expect("pipeline must build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("pipeline must execute");
    assert_eq!(ctx, 15);
}

/// @covers: StepRegistry
#[test]
fn test_step_registry_svc_error_build_fails_on_first_unknown() {
    let mut registry: Box<dyn StepRegistry<i32>> = create_step_registry();
    registry.register("known", Arc::new(AddStep(1)));
    let def = PipelineDefinition {
        steps: vec!["known".to_owned(), "absent".to_owned()],
        ..Default::default()
    };
    match registry.build_pipeline(&def) {
        Err(PipelineError::UnknownStep(name)) => assert_eq!(name, "absent"),
        Err(e) => panic!("expected UnknownStep(absent), got error: {:?}", e),
        Ok(_) => panic!("expected UnknownStep, got Ok"),
    }
}

/// @covers: StepRegistry
#[tokio::test]
async fn test_step_registry_svc_edge_step_registered_multiple_times_only_last_counts() {
    let mut registry: Box<dyn StepRegistry<i32>> = create_step_registry();
    registry.register("s", Arc::new(AddStep(100)));
    registry.register("s", Arc::new(AddStep(1))); // overwrites 100 with 1
    let def = PipelineDefinition { steps: vec!["s".to_owned()], ..Default::default() };
    let pipeline = registry.build_pipeline(&def).expect("must build");
    let mut ctx = 0i32;
    pipeline.execute(&mut ctx).await.expect("must execute");
    assert_eq!(ctx, 1); // last registration wins
}
