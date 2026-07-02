//! End-to-end behavior tests for `ParallelStep` — shared-context visibility, lifecycle
//! events, panic handling, and the adapter-nesting pattern documented in RFC-002.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use edge_domain_event::{EventBus, InProcessEventBus};
use edge_domain_pipeline::{
    ContextMutationRequest, ParallelStepBuilder, ParallelStepError, ParallelStepSvc, Pipeline,
    PipelineBuilder, PipelineSvc, Step, StepNameRequest, StepNameResponse,
};

// ── Ctx: Clone + Arc-wrapped fields for cross-branch visibility (RFC-002's core claim) ──

#[derive(Clone, Default)]
struct SharedCtx {
    log: Arc<Mutex<Vec<String>>>,
}

struct LoggingStep(&'static str);

#[async_trait::async_trait]
impl Step for LoggingStep {
    type Ctx = SharedCtx;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, SharedCtx>) -> Result<(), String> {
        req.ctx.log.lock().expect("lock").push(self.0.to_string());
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: self.0.to_string(),
        })
    }
}

/// @covers: ParallelStep::execute
#[tokio::test]
async fn test_shared_arc_field_visible_after_fan_out_happy() {
    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(LoggingStep("a"))
            .with(LoggingStep("b"))
            .with(LoggingStep("c")),
    );
    let ctx = SharedCtx::default();
    let mut ctx_for_call = ctx.clone();
    let result = step
        .execute(ContextMutationRequest {
            ctx: &mut ctx_for_call,
        })
        .await;
    assert!(result.is_ok());

    let mut names = ctx.log.lock().expect("lock").clone();
    names.sort();
    assert_eq!(
        names,
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        "writes through the Arc<Mutex<..>> field must be visible on the original ctx \
         even though each branch ran against its own clone"
    );
}

// ── Panic handling ────────────────────────────────────────────────────────────────

struct PanicStep;

#[async_trait::async_trait]
impl Step for PanicStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        panic!("intentional test panic");
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: "panics".to_string(),
        })
    }
}

struct OkStep;

#[async_trait::async_trait]
impl Step for OkStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: "ok".to_string(),
        })
    }
}

/// @covers: ParallelStep::execute
#[tokio::test]
async fn test_panicking_branch_reported_as_failure_error() {
    let step = ParallelStepSvc::build(ParallelStepBuilder::new().with(OkStep).with(PanicStep));
    let mut ctx = 0;
    let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
    let err = result.expect_err("a panicking branch must surface as a failure, not vanish");
    assert!(
        matches!(
            err.failures.as_slice(),
            [edge_domain_pipeline::ParallelBranchFailure::Panicked]
        ),
        "the surviving OkStep must not be reported as a failure, and the panic must be \
         reported as Panicked, not silently dropped"
    );
}

// ── Lifecycle events ────────────────────────────────────────────────────────────────

/// @covers: ParallelStep::execute
#[tokio::test]
async fn test_lifecycle_events_fire_per_branch_edge() {
    let bus = Arc::new(InProcessEventBus::new(16));
    let mut receiver = bus.subscribe();

    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(LoggingStep("a"))
            .with(LoggingStep("b"))
            .emit_lifecycle_events(true)
            .with_event_bus(bus.clone() as Arc<dyn EventBus>),
    );
    let mut ctx = SharedCtx::default();
    step.execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("must succeed");

    let mut event_types = Vec::new();
    for _ in 0..4 {
        let event = tokio::time::timeout(Duration::from_secs(1), receiver.recv())
            .await
            .expect("must receive an event before the timeout")
            .expect("event bus recv must not error");
        event_types.push(event.event_type().to_string());
    }
    let started = event_types
        .iter()
        .filter(|t| *t == "pipeline.step_started")
        .count();
    let completed = event_types
        .iter()
        .filter(|t| *t == "pipeline.step_completed")
        .count();
    assert_eq!(started, 2, "one StepStarted per branch");
    assert_eq!(completed, 2, "one StepCompleted per branch");
}

/// @covers: ParallelStep::execute
#[tokio::test]
async fn test_lifecycle_events_not_emitted_when_disabled_edge() {
    let bus = Arc::new(InProcessEventBus::new(16));
    let mut receiver = bus.subscribe();

    let step = ParallelStepSvc::build(
        ParallelStepBuilder::new()
            .with(LoggingStep("a"))
            .with_event_bus(bus.clone() as Arc<dyn EventBus>), // emit_lifecycle_events left at default (false)
    );
    let mut ctx = SharedCtx::default();
    step.execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .expect("must succeed");

    let outcome = tokio::time::timeout(Duration::from_millis(100), receiver.recv()).await;
    assert!(
        outcome.is_err(),
        "no event should be published when emit_lifecycle_events is false"
    );
}

// ── Adapter-nesting pattern (RFC-002 acceptance criteria) ────────────────────────────

/// Wraps a built `ParallelStep` (error type `ParallelStepError<E>`) so it can be composed
/// into an outer `Pipeline<Ctx, E>` whose error type is bare `E` — the same
/// consumer-written-adapter pattern already used for `Pipeline`-as-`Step` nesting
/// (`PipelineAsStep` in `tests/default_pipeline_int_test.rs`).
struct ParallelAsStep {
    inner: Box<dyn Step<Ctx = i32, ExecutionError = ParallelStepError<String>>>,
}

#[async_trait::async_trait]
impl Step for ParallelAsStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        self.inner
            .execute(req)
            .await
            .map_err(|e| format!("{} branch(es) failed", e.failures.len()))
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, edge_domain_pipeline::PipelineError<String>> {
        Ok(StepNameResponse {
            name: "parallel-as-step".to_string(),
        })
    }
}

/// @covers: ParallelStep::execute
#[tokio::test]
async fn test_parallel_as_step_composes_into_outer_pipeline_happy() {
    let parallel: Box<dyn Step<Ctx = i32, ExecutionError = ParallelStepError<String>>> =
        ParallelStepSvc::build(ParallelStepBuilder::new().with(OkStep).with(OkStep));
    let adapter = ParallelAsStep { inner: parallel };

    let pipeline: Box<dyn Pipeline<Ctx = i32, E = String, Request = i32, Response = i32>> =
        PipelineSvc::build(PipelineBuilder::new().with(adapter));

    let mut ctx = 0;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(
        result.is_ok(),
        "an all-succeeding parallel fan-out nested via the adapter must not fail the outer pipeline"
    );
}
