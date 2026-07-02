//! Integration tests — `StepRegistrySvc` construction surface.
//! @covers StepRegistrySvc::create, StepRegistrySvc::create_shared
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineAssemblyRequest, PipelineDefinition, PipelineError, Step,
    StepCountRequest, StepNameRequest, StepNameResponse, StepRegistrationRequest, StepRegistrySvc,
};

struct PassStep;

#[async_trait::async_trait]
impl Step for PassStep {
    type Ctx = ();
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, ()>) -> Result<(), String> {
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "pass".to_string(),
        })
    }
}

// ── StepRegistrySvc::create ───────────────────────────────────────────────────

/// @covers: create
#[test]
fn test_create_builds_pipeline_from_registered_steps_happy() {
    let mut registry = StepRegistrySvc::create::<(), String>();
    registry
        .register(StepRegistrationRequest {
            name: "pass".to_string(),
            step: Arc::new(PassStep),
        })
        .expect("must succeed");
    let definition = PipelineDefinition {
        steps: vec!["pass".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(definition),
    });
    assert!(result.is_ok());
    assert_eq!(
        result
            .expect("pipeline must build")
            .pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}

/// @covers: create
#[test]
fn test_create_unknown_step_returns_error() {
    let registry = StepRegistrySvc::create::<(), String>();
    let definition = PipelineDefinition {
        steps: vec!["nonexistent".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(definition),
    });
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::UnknownStep(_))));
}

/// @covers: create
#[test]
fn test_create_empty_steps_list_builds_empty_pipeline_edge() {
    let registry = StepRegistrySvc::create::<(), String>();
    let definition = PipelineDefinition::default();
    let pipeline = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(definition),
        })
        .expect("empty pipeline must build")
        .pipeline;
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

// ── StepRegistrySvc::create_shared ───────────────────────────────────────────

/// @covers: create_shared
#[test]
fn test_create_shared_builds_pipeline_from_registered_steps_happy() {
    let mut registry = StepRegistrySvc::create_shared::<(), String>();
    Arc::get_mut(&mut registry)
        .expect("exclusive ref")
        .register(StepRegistrationRequest {
            name: "pass".to_string(),
            step: Arc::new(PassStep),
        })
        .expect("must succeed");
    let definition = PipelineDefinition {
        steps: vec!["pass".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(definition),
    });
    assert!(result.is_ok());
    assert_eq!(
        result
            .expect("pipeline must build")
            .pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}

/// @covers: create_shared
#[test]
fn test_create_shared_unknown_step_returns_error() {
    let registry = StepRegistrySvc::create_shared::<(), String>();
    let definition = PipelineDefinition {
        steps: vec!["missing".to_string()],
        ..Default::default()
    };
    let result = registry.build_pipeline(PipelineAssemblyRequest {
        definition: Box::new(definition),
    });
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::UnknownStep(_))));
}

/// @covers: create_shared
#[test]
fn test_create_shared_arc_is_cloneable_edge() {
    let registry = StepRegistrySvc::create_shared::<(), String>();
    let clone = Arc::clone(&registry);
    let definition = PipelineDefinition::default();
    let p1 = registry
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(definition.clone()),
        })
        .expect("primary registry build fails on empty definition")
        .pipeline;
    let p2 = clone
        .build_pipeline(PipelineAssemblyRequest {
            definition: Box::new(definition),
        })
        .expect("cloned registry build fails on empty definition")
        .pipeline;
    assert_eq!(
        p1.step_count(StepCountRequest).expect("must succeed").count,
        0
    );
    assert_eq!(
        p2.step_count(StepCountRequest).expect("must succeed").count,
        0
    );
}
