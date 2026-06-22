//! Comprehensive scenario coverage for Pipeline trait.
//! Tests: happy path, error path, edge cases

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineError, AlwaysPassStep, AlwaysFailStep};
use std::sync::Arc;

struct CountingStep;

#[async_trait::async_trait]
impl Step<usize> for CountingStep {
    async fn execute(&self, ctx: &mut usize) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "counter"
    }
}

struct FailAtStep(usize);

#[async_trait::async_trait]
impl Step<usize> for FailAtStep {
    async fn execute(&self, ctx: &mut usize) -> Result<(), PipelineError> {
        *ctx += 1;
        if *ctx == self.0 {
            Err(PipelineError::StepFailed(format!("failed at {}", self.0)))
        } else {
            Ok(())
        }
    }

    fn name(&self) -> &str {
        "fail-at"
    }
}

// Happy path: execute all steps
/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_happy_empty() {
    let pipeline = create_pipeline(vec![]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_happy_single_step() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_happy_multiple_steps() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, 3);
}

// Error path: early exit on failure
#[tokio::test]
async fn test_pipeline_execute_error_first_step() {
    let pipeline = create_pipeline(vec![
        Arc::new(FailAtStep(1)),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_error_middle_step() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(FailAtStep(2)),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_execute_error_last_step() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(FailAtStep(3)),
    ]);
    let mut ctx = 0;
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 3);
}

// Edge cases
/// @covers: step_count
#[tokio::test]
async fn test_pipeline_step_count_happy_zero() {
    let pipeline = create_pipeline(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

#[tokio::test]
async fn test_pipeline_step_count_happy_many() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    assert_eq!(pipeline.step_count(), 5);
}

#[tokio::test]
async fn test_pipeline_is_empty_happy_true() {
    let pipeline = create_pipeline(vec![]);
    assert!(pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_is_empty_happy_false() {
    let pipeline = create_pipeline(vec![
        Arc::new(CountingStep),
    ]);
    assert!(!pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_execute_error_error_message() {
    let pipeline = create_pipeline(vec![
        Arc::new(FailAtStep(1)),
    ]);
    let mut ctx = 0;
    match Pipeline::execute(&pipeline, &mut ctx).await {
        Err(PipelineError::StepFailed(msg)) => assert!(msg.contains("failed")),
        _ => panic!("expected StepFailed"),
    }
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_happy() {
    let pipeline = Box::new(create_pipeline(vec![
        Arc::new(CountingStep),
    ]));
    let mut ctx = 0;
    assert!(Pipeline::execute(pipeline.as_ref(), &mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_error() {
    let pipeline = Box::new(create_pipeline(vec![
        Arc::new(FailAtStep(1)),
    ]));
    let mut ctx = 0;
    assert!(Pipeline::execute(pipeline.as_ref(), &mut ctx).await.is_err());
}
