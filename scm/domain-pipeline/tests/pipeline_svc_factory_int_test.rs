//! Integration tests — `PipelineSvc` construction surface.
//! @covers PipelineSvc::build, PipelineSvc::build_shared
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::marker::PhantomData;
use std::sync::Arc;

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfigLookupRequest, PipelineError,
    PipelineSvc, Step, StepCountRequest,
};

/// Generic over `Ctx`/`E` because this file exercises the step against both
/// `Ctx = ()` and `Ctx = i32` pipelines — a single concrete type can no longer
/// implement `Step` for two different `Ctx`/`E` pairs under associated types.
struct AlwaysPass<Ctx, E>(PhantomData<fn(Ctx, E)>);

impl<Ctx, E> Default for AlwaysPass<Ctx, E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step for AlwaysPass<Ctx, E> {
    type Ctx = Ctx;
    type ExecutionError = E;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Ok(())
    }
}

/// Generic over `Ctx` for the same reason as [`AlwaysPass`]; `E` stays `String`.
struct AlwaysFail<Ctx>(PhantomData<fn(Ctx)>);

impl<Ctx> Default for AlwaysFail<Ctx> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Step for AlwaysFail<Ctx> {
    type Ctx = Ctx;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), String> {
        Err("forced failure".to_string())
    }
}

// ── PipelineSvc::build ────────────────────────────────────────────────────────

/// @covers: build
#[tokio::test]
async fn test_build_with_steps_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with(AlwaysPass::<(), String>::default())
                .with(AlwaysPass::<(), String>::default()),
        );
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: build
#[tokio::test]
async fn test_build_step_failure_propagated_error() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(
            PipelineBuilder::new()
                .with(AlwaysPass::<(), String>::default())
                .with(AlwaysFail::<()>::default()),
        );
    let mut ctx = ();
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::StepFailed(_))));
}

/// @covers: build
#[tokio::test]
async fn test_build_empty_pipeline_succeeds_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(PipelineBuilder::new());
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

// ── PipelineSvc::build_shared ─────────────────────────────────────────────────

/// @covers: build_shared
#[tokio::test]
async fn test_build_shared_with_steps_happy() {
    let pipeline: Arc<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build_shared(PipelineBuilder::new().with(AlwaysPass::<(), String>::default()));
    let mut ctx = ();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: build_shared
#[tokio::test]
async fn test_build_shared_step_failure_propagated_error() {
    let pipeline: Arc<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build_shared(PipelineBuilder::new().with(AlwaysFail::<()>::default()));
    let mut ctx = ();
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(PipelineError::StepFailed(_))));
}

/// @covers: build_shared
#[tokio::test]
async fn test_build_shared_is_cloneable_edge() {
    let pipeline: Arc<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build_shared(PipelineBuilder::new().with(AlwaysPass::<(), String>::default()));
    let clone = Arc::clone(&pipeline);
    let mut ctx = ();
    assert!(clone
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        clone
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count
    );
}

// ── PipelineConfig validation via build ───────────────────────────────────────

/// @covers: build
#[test]
fn test_build_config_timeout_none_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(PipelineBuilder::new());
    assert!(pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config
        .timeout_per_step
        .is_none());
}

/// @covers: build
#[test]
fn test_build_config_abort_on_error_default_happy() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(PipelineBuilder::new());
    assert!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .abort_on_error
    );
}

/// @covers: build
#[test]
fn test_build_with_custom_config_edge() {
    let pipeline: Box<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build(PipelineBuilder::new().abort_on_error(false));
    assert!(
        !pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .abort_on_error
    );
}

/// @covers: build_shared
#[test]
fn test_build_shared_config_reflects_builder_edge() {
    let pipeline: Arc<dyn Pipeline<Ctx = (), E = String, Request = (), Response = ()>> =
        PipelineSvc::build_shared(
            PipelineBuilder::new()
                .abort_on_error(false)
                .emit_lifecycle_events(true),
        );
    let config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert!(!config.abort_on_error);
    assert!(config.emit_lifecycle_events);
}

// ── PipelineBuilder::with ─────────────────────────────────────────────────────

/// @covers: with
#[test]
fn test_with_adds_step_to_builder_happy() {
    let builder: PipelineBuilder<i32, String> =
        PipelineBuilder::new().with(AlwaysPass::<i32, String>::default());
    assert_eq!(builder.steps.len(), 1);
}

/// @covers: with
#[test]
fn test_with_multiple_steps_accumulate_error() {
    let builder: PipelineBuilder<i32, String> = PipelineBuilder::new()
        .with(AlwaysPass::<i32, String>::default())
        .with(AlwaysPass::<i32, String>::default())
        .with(AlwaysFail::<i32>::default());
    assert_eq!(builder.steps.len(), 3);
}

/// @covers: with
#[test]
fn test_with_empty_then_one_step_edge() {
    let builder: PipelineBuilder<i32, String> = PipelineBuilder::new();
    assert!(builder.steps.is_empty());
    let builder = builder.with(AlwaysPass::<i32, String>::default());
    assert_eq!(builder.steps.len(), 1);
}

// ── PipelineBuilder::with_shared ──────────────────────────────────────────────

/// @covers: with_shared
#[test]
fn test_with_shared_adds_arc_step_happy() {
    let step = Arc::new(AlwaysPass::<(), String>::default());
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new().with_shared(step);
    assert_eq!(builder.steps.len(), 1);
}

/// @covers: with_shared
#[test]
fn test_with_shared_multiple_arcs_accumulate_error() {
    let s1: Arc<dyn Step<Ctx = (), ExecutionError = String>> =
        Arc::new(AlwaysPass::<(), String>::default());
    let s2: Arc<dyn Step<Ctx = (), ExecutionError = String>> =
        Arc::new(AlwaysFail::<()>::default());
    let builder: PipelineBuilder<(), String> =
        PipelineBuilder::new().with_shared(s1).with_shared(s2);
    assert_eq!(builder.steps.len(), 2);
}

/// @covers: with_shared
#[test]
fn test_with_shared_same_arc_twice_edge() {
    let step: Arc<dyn Step<Ctx = (), ExecutionError = String>> =
        Arc::new(AlwaysPass::<(), String>::default());
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new()
        .with_shared(Arc::clone(&step))
        .with_shared(step);
    assert_eq!(builder.steps.len(), 2);
}

// ── PipelineBuilder::with_timeout ─────────────────────────────────────────────

/// @covers: with_timeout
#[test]
fn test_with_timeout_sets_duration_happy() {
    use std::time::Duration;
    let dur = Duration::from_secs(5);
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new().with_timeout(dur);
    assert_eq!(builder.config.timeout_per_step, Some(dur));
}

/// @covers: with_timeout
#[test]
fn test_with_timeout_zero_duration_stored_error() {
    use std::time::Duration;
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new().with_timeout(Duration::ZERO);
    assert_eq!(builder.config.timeout_per_step, Some(Duration::ZERO));
}

/// @covers: with_timeout
#[test]
fn test_with_timeout_overrides_previous_edge() {
    use std::time::Duration;
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new()
        .with_timeout(Duration::from_secs(1))
        .with_timeout(Duration::from_secs(10));
    assert_eq!(
        builder.config.timeout_per_step,
        Some(Duration::from_secs(10))
    );
}

// ── PipelineBuilder::emit_lifecycle_events ────────────────────────────────────

/// @covers: emit_lifecycle_events
#[test]
fn test_emit_lifecycle_events_enables_flag_happy() {
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new().emit_lifecycle_events(true);
    assert!(builder.config.emit_lifecycle_events);
}

/// @covers: emit_lifecycle_events
#[test]
fn test_emit_lifecycle_events_can_be_disabled_error() {
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new()
        .emit_lifecycle_events(true)
        .emit_lifecycle_events(false);
    assert!(!builder.config.emit_lifecycle_events);
}

/// @covers: emit_lifecycle_events
#[test]
fn test_emit_lifecycle_events_default_is_false_edge() {
    let builder: PipelineBuilder<(), String> = PipelineBuilder::new();
    assert!(!builder.config.emit_lifecycle_events);
}
