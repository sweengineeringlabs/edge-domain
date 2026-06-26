//! Comprehensive scenario coverage for Pipeline trait.
//! Tests: happy path, error path, edge cases

use edge_domain_pipeline::{Pipeline, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step};
use std::sync::Arc;

struct CountingStep;

#[async_trait::async_trait]
impl Step<usize> for CountingStep {
    async fn execute(&self, ctx: &mut usize) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }
    fn name(&self) -> &str { "counter" }
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
    fn name(&self) -> &str { "fail-at" }
}

// Happy path: execute all steps
/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_empty_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    let mut ctx = 0;
    assert!(pipeline.run(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_single_step_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    let mut ctx = 0;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_multiple_steps_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(
        PipelineBuilder::new().with(CountingStep).with(CountingStep).with(CountingStep),
    );
    let mut ctx = 0;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 3);
}

// Error path: early exit on failure
#[tokio::test]
async fn test_pipeline_execute_first_step_error() {
    let steps: Vec<Arc<dyn Step<usize>>> = vec![
        Arc::new(FailAtStep(1)),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ];
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default(), event_bus: None });
    let mut ctx = 0;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_middle_step_error() {
    let steps: Vec<Arc<dyn Step<usize>>> = vec![
        Arc::new(CountingStep),
        Arc::new(FailAtStep(2)),
        Arc::new(CountingStep),
    ];
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default(), event_bus: None });
    let mut ctx = 0;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_execute_last_step_error() {
    let steps: Vec<Arc<dyn Step<usize>>> = vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(FailAtStep(3)),
    ];
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default(), event_bus: None });
    let mut ctx = 0;
    let result = pipeline.run(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, 3);
}

// Edge cases
/// @covers: step_count
#[tokio::test]
async fn test_pipeline_step_count_zero_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    assert_eq!(pipeline.step_count(), 0);
}

#[tokio::test]
async fn test_pipeline_step_count_many_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(CountingStep).with(CountingStep).with(CountingStep)
            .with(CountingStep).with(CountingStep),
    );
    assert_eq!(pipeline.step_count(), 5);
}

#[tokio::test]
async fn test_pipeline_is_empty_true_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    assert!(pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_is_empty_false_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(!pipeline.is_empty());
}

#[tokio::test]
async fn test_pipeline_execute_error_message_error() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(FailAtStep(1)));
    let mut ctx = 0;
    match pipeline.run(&mut ctx).await {
        Err(PipelineError::StepFailed(msg)) => assert!(msg.contains("failed")),
        _ => panic!("expected StepFailed"),
    }
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    let mut ctx = 0usize;
    assert!(pipeline.run(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_error_happy() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(FailAtStep(1)));
    let mut ctx = 0usize;
    assert!(pipeline.run(&mut ctx).await.is_err());
}

// Scenario coverage for step_count
/// @covers: step_count
#[test]
fn test_step_count_empty_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    assert_eq!(pipeline.step_count(), 0);
}

#[test]
fn test_step_count_single_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert_eq!(pipeline.step_count(), 1);
}

#[test]
fn test_step_count_multiple_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(
        PipelineBuilder::new().with(CountingStep).with(CountingStep).with(CountingStep),
    );
    assert_eq!(pipeline.step_count(), 3);
}

#[test]
fn test_step_count_max_edge() {
    let steps: Vec<Arc<dyn Step<usize>>> = (0..100).map(|_| Arc::new(CountingStep) as Arc<dyn Step<usize>>).collect();
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default(), event_bus: None });
    assert_eq!(pipeline.step_count(), 100);
}

// Scenario coverage for is_empty
/// @covers: is_empty
#[test]
fn test_is_empty_empty_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    assert!(pipeline.is_empty());
}

#[test]
fn test_is_empty_single_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(!pipeline.is_empty());
}

#[test]
fn test_is_empty_multiple_happy_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(
        PipelineBuilder::new().with(CountingStep).with(CountingStep),
    );
    assert!(!pipeline.is_empty());
}

#[test]
fn test_is_empty_one_edge() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(!pipeline.is_empty());
}

// Error case for step_count with stress-tested max
#[test]
fn test_step_count_stress_edge() {
    let steps: Vec<Arc<dyn Step<usize>>> = (0..1000).map(|_| Arc::new(CountingStep) as Arc<dyn Step<usize>>).collect();
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default(), event_bus: None });
    assert_eq!(pipeline.step_count(), 1000);
}

#[test]
fn test_pipeline_config_constraint_error() {
    let pipeline: Box<dyn Pipeline<usize>> = PipelineSvc::build(PipelineBuilder::<usize>::new());
    assert_eq!(pipeline.config().timeout_per_step, None);
}
