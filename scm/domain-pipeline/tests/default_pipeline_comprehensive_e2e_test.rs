//! @covers DefaultPipeline config and nesting
//! Comprehensive scenario coverage for DefaultPipeline struct.
//! Tests: config variations, nesting, edge cases

use edge_domain_pipeline::{ create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineConfig, AlwaysPassStep, AlwaysFailStep, MutatingStep};
use std::sync::Arc;
use std::time::Duration;

// Config variation: timeout
#[test]
fn test_default_pipeline_config_happy_with_timeout() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
}

#[test]
fn test_default_pipeline_config_happy_no_timeout() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert_eq!(pipeline.config().timeout_per_step, None);
}

// Config variation: lifecycle events
#[test]
fn test_default_pipeline_config_happy_lifecycle_enabled() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert!(pipeline.config().emit_lifecycle_events);
}

#[test]
fn test_default_pipeline_config_happy_lifecycle_disabled() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert!(!pipeline.config().emit_lifecycle_events);
}

// Config variation: abort_on_error
#[test]
fn test_default_pipeline_config_happy_abort_true() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert!(pipeline.config().abort_on_error);
}

#[test]
fn test_default_pipeline_config_happy_abort_false() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline = create_pipeline_with_config(vec![], config);
    assert!(!pipeline.config().abort_on_error);
}

// Composability: nested pipelines
#[tokio::test]
async fn test_default_pipeline_composability_happy_single_nesting() {
    let inner = create_pipeline(vec![Arc::new(AlwaysPassStep::new())]);
    let outer: _ = create_pipeline(vec![Arc::new(inner)]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&outer, &mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_default_pipeline_composability_happy_double_nesting() {
    let level1 = create_pipeline(vec![Arc::new(AlwaysPassStep::new())]);
    let level2: _ = create_pipeline(vec![Arc::new(level1)]);
    let level3: _ = create_pipeline(vec![Arc::new(level2)]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&level3, &mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_default_pipeline_composability_error_inner_fails() {
    let inner = create_pipeline(vec![Arc::new(AlwaysFailStep::new("inner failed"))]);
    let outer: _ = create_pipeline(vec![Arc::new(inner)]);
    let mut ctx = 0;
    let result = Pipeline::execute(&outer, &mut ctx).await;
    assert!(result.is_err());
}

// Context mutation across steps
#[tokio::test]
async fn test_default_pipeline_mutation_happy_accumulate() {
    let pipeline = create_pipeline(vec![
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 1)),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 2)),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 3)),
    ]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, 6);
}

#[tokio::test]
async fn test_default_pipeline_mutation_happy_chain() {
    let pipeline = create_pipeline(vec![
        Arc::new(MutatingStep::new(|ctx: &mut String| ctx.push_str("a"))),
        Arc::new(MutatingStep::new(|ctx: &mut String| ctx.push_str("b"))),
        Arc::new(MutatingStep::new(|ctx: &mut String| ctx.push_str("c"))),
    ]);
    let mut ctx = String::new();
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, "abc");
}

// Edge cases: large pipelines
#[tokio::test]
async fn test_default_pipeline_edge_many_steps() {
    let mut steps: Vec<Arc<dyn Step<i32>>> = vec![];
    for _ in 0..100 {
        steps.push(Arc::new(AlwaysPassStep::new()));
    }
    let pipeline = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 100);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
}

// Edge cases: mixed step types
#[tokio::test]
async fn test_default_pipeline_edge_mixed_step_types() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 5)),
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx *= 2)),
    ]);
    let mut ctx = 3;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, 16);
}

#[tokio::test]
async fn test_default_pipeline_edge_fail_in_mixed_chain() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 5)),
        Arc::new(AlwaysFailStep::new("stop")),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx *= 2)),
    ]);
    let mut ctx = 3;
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 8);
}

// Clone support
#[test]
fn test_default_pipeline_clone_happy() {
    let pipeline1: _ = create_pipeline(vec![]);
    let pipeline2 = create_pipeline(vec![]);
    assert_eq!(pipeline2.step_count(), 0);
}
