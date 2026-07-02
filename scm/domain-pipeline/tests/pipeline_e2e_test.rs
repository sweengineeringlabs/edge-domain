//! End-to-end coverage for the [`Pipeline`] trait's public method surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineConfigResponse, PipelineEmptinessRequest, PipelineError, PipelineSvc, StepCountRequest,
    StepCountResponse, StepSvc,
};
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

/// Minimal `Pipeline` implementor used only to reach the `new_builder` default
/// method — it requires `Self: Sized`, so it is unreachable through
/// `Box<dyn Pipeline<..>>` and must be called on a concrete, in-scope type.
struct NewBuilderProbe;

impl Service for NewBuilderProbe {
    type Request = i32;
    type Response = i32;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "new-builder-probe".to_string(),
        })
    }

    fn execute(&self, ctx: i32) -> BoxFuture<'_, Result<i32, ServiceError>> {
        Box::pin(async move { Ok(ctx) })
    }
}

#[async_trait::async_trait]
impl Pipeline for NewBuilderProbe {
    type Ctx = i32;
    type E = String;

    async fn run(
        &self,
        _req: ContextMutationRequest<'_, i32>,
    ) -> Result<(), PipelineError<String>> {
        Ok(())
    }

    fn step_count(
        &self,
        _req: StepCountRequest,
    ) -> Result<StepCountResponse, PipelineError<String>> {
        Ok(StepCountResponse { count: 0 })
    }

    fn config(
        &self,
        _req: PipelineConfigLookupRequest,
    ) -> Result<PipelineConfigResponse, PipelineError<String>> {
        Ok(PipelineConfigResponse {
            config: PipelineConfig::default(),
        })
    }
}

/// @covers: run
#[tokio::test]
async fn test_run_happy_mutates_context_through_registered_steps() {
    let step = StepSvc::noop_shared::<i32, String>();
    let pipeline = PipelineSvc::build(PipelineBuilder::new().with_shared(step));
    let mut ctx = 41;
    pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("run must succeed");
    assert_eq!(ctx, 41, "noop step must leave the context unchanged");
}

/// @covers: run
#[tokio::test]
async fn test_run_error_step_failure_is_propagated() {
    struct FailingStep;
    #[async_trait::async_trait]
    impl edge_domain_pipeline::Step for FailingStep {
        type Ctx = i32;
        type ExecutionError = String;

        async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
            Err("boom".to_string())
        }
    }
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String>::new().with(FailingStep));
    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err(), "a failing step must surface as Err");
}

/// @covers: step_count
#[test]
fn test_step_count_edge_reflects_registered_step_total() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::<i32, String>::new()
            .with(StepCountProbe)
            .with(StepCountProbe)
            .with(StepCountProbe),
    );
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("step_count must succeed")
            .count,
        3
    );
}

struct StepCountProbe;
#[async_trait::async_trait]
impl edge_domain_pipeline::Step for StepCountProbe {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }
}

/// @covers: is_empty
#[test]
fn test_is_empty_happy_true_for_empty_pipeline() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String>::new());
    assert!(
        pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("is_empty must succeed")
            .empty
    );
}

/// @covers: is_empty
#[test]
fn test_is_empty_error_false_once_a_step_is_registered() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String>::new().with(StepCountProbe));
    assert!(
        !pipeline
            .is_empty(PipelineEmptinessRequest)
            .expect("is_empty must succeed")
            .empty
    );
}

/// @covers: config
#[test]
fn test_config_happy_reports_builder_supplied_values() {
    let pipeline = PipelineSvc::build(
        PipelineBuilder::<i32, String>::new()
            .with_timeout(Duration::from_secs(7))
            .abort_on_error(false),
    );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("config must succeed")
        .config;
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(7)));
    assert!(!config.abort_on_error);
}

/// @covers: config
#[test]
fn test_config_edge_defaults_when_builder_untouched() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String>::new());
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("config must succeed")
        .config;
    assert!(config.timeout_per_step.is_none());
    assert!(config.abort_on_error);
}

/// @covers: new_builder
#[test]
fn test_new_builder_happy_starts_with_no_steps() {
    let builder: PipelineBuilder<i32, String> = NewBuilderProbe::new_builder();
    assert!(builder.steps.is_empty());
}

/// @covers: new_builder
#[test]
fn test_new_builder_edge_default_config_matches_pipeline_config_default() {
    let builder: PipelineBuilder<i32, String> = NewBuilderProbe::new_builder();
    assert!(builder.config.abort_on_error);
    assert!(builder.config.timeout_per_step.is_none());
}
