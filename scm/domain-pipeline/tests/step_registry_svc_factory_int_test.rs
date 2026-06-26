//! Integration tests — `StepRegistrySvc` construction surface.
//! @covers StepRegistrySvc::create, StepRegistrySvc::create_shared

use std::sync::Arc;

use edge_domain_pipeline::{PipelineDefinition, PipelineError, Step, StepRegistrySvc};

struct PassStep;

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for PassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), E> {
        Ok(())
    }
    fn name(&self) -> &str { "pass" }
}

// ── StepRegistrySvc::create ───────────────────────────────────────────────────

/// @covers: create
#[test]
fn test_create_builds_pipeline_from_registered_steps_happy() {
    let mut registry = StepRegistrySvc::create::<(), String>();
    registry.register("pass", Arc::new(PassStep));
    let definition = PipelineDefinition {
        steps: vec!["pass".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(&definition);
    assert!(result.is_ok());
    assert_eq!(result.expect("pipeline must build").step_count(), 1);
}

/// @covers: create
#[test]
fn test_create_unknown_step_returns_error() {
    let registry = StepRegistrySvc::create::<(), String>();
    let definition = PipelineDefinition {
        steps: vec!["nonexistent".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(&definition);
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::UnknownStep(_))));
}

/// @covers: create
#[test]
fn test_create_empty_steps_list_builds_empty_pipeline_edge() {
    let registry = StepRegistrySvc::create::<(), String>();
    let definition = PipelineDefinition::default();
    let pipeline = registry.build_pipeline(&definition).expect("empty pipeline must build");
    assert_eq!(pipeline.step_count(), 0);
}

// ── StepRegistrySvc::create_shared ───────────────────────────────────────────

/// @covers: create_shared
#[test]
fn test_create_shared_builds_pipeline_from_registered_steps_happy() {
    let mut registry = StepRegistrySvc::create_shared::<(), String>();
    Arc::get_mut(&mut registry)
        .expect("exclusive ref")
        .register("pass", Arc::new(PassStep));
    let definition = PipelineDefinition {
        steps: vec!["pass".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(&definition);
    assert!(result.is_ok());
    assert_eq!(result.expect("pipeline must build").step_count(), 1);
}

/// @covers: create_shared
#[test]
fn test_create_shared_unknown_step_returns_error() {
    let registry = StepRegistrySvc::create_shared::<(), String>();
    let definition = PipelineDefinition {
        steps: vec!["missing".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(&definition);
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::UnknownStep(_))));
}

/// @covers: create_shared
#[test]
fn test_create_shared_arc_is_cloneable_edge() {
    let registry = StepRegistrySvc::create_shared::<(), String>();
    let clone = Arc::clone(&registry);
    let definition = PipelineDefinition::default();
    let p1 = registry.build_pipeline(&definition).expect("primary registry build fails on empty definition");
    let p2 = clone.build_pipeline(&definition).expect("cloned registry build fails on empty definition");
    assert_eq!(p1.step_count(), 0);
    assert_eq!(p2.step_count(), 0);
}
