//! Comprehensive scenario coverage for Pipeline trait.
//! Tests: happy path, error path, edge cases

use edge_domain_pipeline::{create_pipeline, Pipeline, Step, PipelineError};
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
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![]);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_happy_single_step() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_happy_multiple_steps() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 3);
}

// Error path: early exit on failure
#[tokio::test]
async fn test_pipeline_execute_error_first_step() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(FailAtStep(1)),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_error_middle_step() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(FailAtStep(2)),
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0;
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_execute_error_last_step() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(FailAtStep(3)),
    ]);
    let mut ctx = 0;
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 3);
}

// Edge cases
/// @covers: step_count
#[tokio::test]
async fn test_pipeline_step_count_happy_zero() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

#[tokio::test]
async fn test_pipeline_step_count_happy_many() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
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
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![]);
    assert!(pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_is_empty_happy_false() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
    ]);
    assert!(!pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_execute_error_error_message() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(FailAtStep(1)),
    ]);
    let mut ctx = 0;
    match pipeline.execute(&mut ctx).await {
        Err(PipelineError::StepFailed(msg)) => assert!(msg.contains("failed")),
        _ => panic!("expected StepFailed"),
    }
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
    ]);
    let mut ctx = 0usize;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_error() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(FailAtStep(1)),
    ]);
    let mut ctx = 0usize;
    assert!(pipeline.execute(&mut ctx).await.is_err());
}

// Scenario coverage for step_count
/// @covers: step_count
#[test]
fn test_step_count_empty_happy() {
    let pipeline: _ = create_pipeline::<usize>(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_step_count_single_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![Arc::new(CountingStep)]);
    assert_eq!(pipeline.step_count(), 1);
}

#[test]
fn test_step_count_multiple_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    assert_eq!(pipeline.step_count(), 3);
}

#[test]
fn test_step_count_edge_max() {
    let steps: Vec<_> = (0..100).map(|_| Arc::new(CountingStep) as Arc<dyn Step<usize>>).collect();
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 100);
}

// Scenario coverage for is_empty
/// @covers: is_empty
#[test]
fn test_is_empty_empty_happy() {
    let pipeline: _ = create_pipeline::<usize>(vec![]);
    assert!(pipeline.is_empty());
}

#[test]
fn test_is_empty_single_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![Arc::new(CountingStep)]);
    assert!(!pipeline.is_empty());
}

#[test]
fn test_is_empty_multiple_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ]);
    assert!(!pipeline.is_empty());
}

#[test]
fn test_is_empty_edge_one() {
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(vec![Arc::new(CountingStep)]);
    assert!(!pipeline.is_empty());
}

// Error case for step_count with stress-tested max
#[test]
fn test_step_count_edge_stress() {
    let steps: Vec<_> = (0..1000).map(|_| Arc::new(CountingStep) as Arc<dyn Step<usize>>).collect();
    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 1000);
}
