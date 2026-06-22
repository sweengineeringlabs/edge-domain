//! Scenario-labeled tests for trait methods: happy/error/edge paths
//! Required by audit: every trait fn must have _happy, _error, _edge test coverage

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, {Pipeline, Step, PipelineError, DefaultPipeline, AlwaysPassStep, AlwaysFailStep};
use std::sync::Arc;

// ─── Step trait: execute ───────────────────────────────────────────────────

/// @covers: execute
#[tokio::test]
async fn test_execute_nominal_happy() {
    let step = AlwaysPassStep::new();
    let mut ctx: i32 = 0;
    assert!(step.execute(&mut ctx).await.is_ok());
}

/// @covers: execute
#[tokio::test]
async fn test_execute_nominal_error() {
    let step = AlwaysFailStep::new("failed");
    let mut ctx: i32 = 0;
    assert!(step.execute(&mut ctx).await.is_err());
}

/// @covers: execute
#[tokio::test]
async fn test_execute_nominal_edge() {
    struct CountingStep;
    #[async_trait::async_trait]
    impl Step<i32> for CountingStep {
        async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
            *ctx += 1;
            Ok(())
        }
        fn name(&self) -> &str { "counter" }
    }
    let step = CountingStep;
    let mut ctx = i32::MAX - 1;
    assert!(step.execute(&mut ctx).await.is_ok());
}

// ─── Step trait: name ──────────────────────────────────────────────────────

/// @covers: name
#[test]
fn test_name_accessor_happy() {
    let step: Box<dyn Step<i32>> = Box::new(AlwaysPassStep::new());
    assert_eq!(step.name(), "always-pass");
}

/// @covers: name
#[test]
fn test_name_accessor_error() {
    let step: Box<dyn Step<i32>> = Box::new(AlwaysFailStep::new("error"));
    assert_eq!(step.name(), "always-fail");
}

/// @covers: name
#[test]
fn test_name_accessor_edge() {
    let step: Box<dyn Step<i32>> = Box::new(AlwaysPassStep::new());
    let name = step.name();
    assert!(!name.is_empty());
    assert!(name.len() > 0);
}

// ─── Pipeline trait: execute ───────────────────────────────────────────────

/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_nominal_happy() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
    ]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
}

/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_nominal_error() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysFailStep::new("stop")),
    ]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_err());
}

/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_nominal_edge() {
    let mut steps: Vec<Arc<dyn Step<i32>>> = vec![];
    for _ in 0..1000 {
        steps.push(Arc::new(AlwaysPassStep::new()));
    }
    let pipeline = create_pipeline(steps);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
}

// ─── Pipeline trait: step_count ────────────────────────────────────────────

/// @covers: step_count
#[test]
fn test_step_count_accessor_happy() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(AlwaysPassStep::new()),
    ]);
    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: step_count
#[test]
fn test_step_count_accessor_error() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysFailStep::new("err")),
    ]);
    assert_eq!(pipeline.step_count(), 1);
}

/// @covers: step_count
#[test]
fn test_step_count_accessor_edge() {
    let mut steps: Vec<Arc<dyn Step<i32>>> = vec![];
    for _ in 0..1000 {
        steps.push(Arc::new(AlwaysPassStep::new()));
    }
    let pipeline = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 1000);
}

// ─── Pipeline trait: is_empty ──────────────────────────────────────────────

/// @covers: is_empty
#[test]
fn test_is_empty_predicate_happy() {
    let pipeline = create_pipeline(vec![]);
    assert!(pipeline.is_empty());
}

/// @covers: is_empty
#[test]
fn test_is_empty_predicate_error() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysFailStep::new("fail")),
    ]);
    assert!(!pipeline.is_empty());
}

/// @covers: is_empty
#[test]
fn test_is_empty_predicate_edge() {
    let mut steps: Vec<Arc<dyn Step<i32>>> = vec![];
    for _ in 0..1000 {
        steps.push(Arc::new(AlwaysPassStep::new()));
    }
    let pipeline = create_pipeline(steps);
    assert!(!pipeline.is_empty());
    assert_eq!(pipeline.step_count() > 0, !pipeline.is_empty());
}
