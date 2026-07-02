//! Comprehensive scenario coverage for Pipeline trait.
//! Tests: happy path, error path, edge cases
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineEmptinessRequest, PipelineError, PipelineSvc, Step, StepCountRequest, StepNameRequest,
    StepNameResponse,
};
use std::sync::Arc;

struct CountingStep;

#[async_trait::async_trait]
impl Step for CountingStep {
    type Ctx = usize;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, usize>) -> Result<(), String> {
        *req.ctx += 1;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "counter".to_string(),
        })
    }
}

struct FailAtStep(usize);

#[async_trait::async_trait]
impl Step for FailAtStep {
    type Ctx = usize;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, usize>) -> Result<(), String> {
        *req.ctx += 1;
        if *req.ctx == self.0 {
            Err(format!("failed at {}", self.0))
        } else {
            Ok(())
        }
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "fail-at".to_string(),
        })
    }
}

// Happy path: execute all steps
/// @covers: execute
#[tokio::test]
async fn test_pipeline_execute_empty_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_execute_single_step_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_multiple_steps_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with(CountingStep)
                .with(CountingStep)
                .with(CountingStep),
        );
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 3);
}

// Error path: early exit on failure
#[tokio::test]
async fn test_pipeline_execute_first_step_error() {
    let steps: Vec<Arc<dyn Step<Ctx = usize, ExecutionError = String>>> = vec![
        Arc::new(FailAtStep(1)),
        Arc::new(CountingStep),
        Arc::new(CountingStep),
    ];
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_execute_middle_step_error() {
    let steps: Vec<Arc<dyn Step<Ctx = usize, ExecutionError = String>>> = vec![
        Arc::new(CountingStep),
        Arc::new(FailAtStep(2)),
        Arc::new(CountingStep),
    ];
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, 2);
}

#[tokio::test]
async fn test_pipeline_execute_last_step_error() {
    let steps: Vec<Arc<dyn Step<Ctx = usize, ExecutionError = String>>> = vec![
        Arc::new(CountingStep),
        Arc::new(CountingStep),
        Arc::new(FailAtStep(3)),
    ];
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, 3);
}

// Edge cases
/// @covers: step_count
#[tokio::test]
async fn test_pipeline_step_count_zero_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[tokio::test]
async fn test_pipeline_step_count_many_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with(CountingStep)
                .with(CountingStep)
                .with(CountingStep)
                .with(CountingStep)
                .with(CountingStep),
        );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        5
    );
}

#[tokio::test]
async fn test_pipeline_is_empty_true_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[tokio::test]
async fn test_pipeline_is_empty_false_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[tokio::test]
async fn test_pipeline_execute_error_message_error() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(FailAtStep(1)));
    let mut ctx = 0;
    match pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await {
        Err(PipelineError::StepFailed(e)) => assert!(e.cause.contains("failed")),
        _ => panic!("expected StepFailed"),
    }
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    let mut ctx = 0usize;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 1);
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_error_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(FailAtStep(1)));
    let mut ctx = 0usize;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_err());
}

// Scenario coverage for step_count
/// @covers: step_count
#[test]
fn test_step_count_empty_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

#[test]
fn test_step_count_single_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1
    );
}

#[test]
fn test_step_count_multiple_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with(CountingStep)
                .with(CountingStep)
                .with(CountingStep),
        );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        3
    );
}

#[test]
fn test_step_count_max_edge() {
    let steps: Vec<Arc<dyn Step<Ctx = usize, ExecutionError = String>>> = (0..100)
        .map(|_| Arc::new(CountingStep) as Arc<dyn Step<Ctx = usize, ExecutionError = String>>)
        .collect();
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        100
    );
}

// Scenario coverage for is_empty
/// @covers: is_empty
#[test]
fn test_is_empty_empty_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_is_empty_single_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_is_empty_multiple_happy_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep).with(CountingStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

#[test]
fn test_is_empty_one_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::new().with(CountingStep));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("must succeed")
            .empty
    );
}

// Error case for step_count with stress-tested max
#[test]
fn test_step_count_stress_edge() {
    let steps: Vec<Arc<dyn Step<Ctx = usize, ExecutionError = String>>> = (0..1000)
        .map(|_| Arc::new(CountingStep) as Arc<dyn Step<Ctx = usize, ExecutionError = String>>)
        .collect();
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        1000
    );
}

#[test]
fn test_pipeline_config_constraint_error() {
    let pipeline: Box<dyn Pipeline<Ctx = usize, E = String, Request = usize, Response = usize>> =
        PipelineSvc::build(PipelineBuilder::<usize, String>::new());
    assert_eq!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .timeout_per_step,
        None
    );
}
