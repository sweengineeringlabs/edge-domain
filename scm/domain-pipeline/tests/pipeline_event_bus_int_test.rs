//! Integration tests — lifecycle event emission (ADR-048 Phase 2).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use edge_domain_event::{EventBus, InProcessEventBus};
use edge_domain_pipeline::{PipelineBuilder, PipelineError, PipelineSvc, Step};

// ── shared test steps ─────────────────────────────────────────────────────────

struct OkStep;

#[async_trait::async_trait]
impl Step<i32> for OkStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "ok-step"
    }
}

struct ErrStep;

#[async_trait::async_trait]
impl Step<i32> for ErrStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed("injected".into()))
    }

    fn name(&self) -> &str {
        "err-step"
    }
}

// ── happy path ────────────────────────────────────────────────────────────────

/// @covers: DefaultPipeline::run — emit_lifecycle_events = true, step succeeds
#[tokio::test]
async fn test_pipeline_emits_step_started_and_completed_on_success_happy() {
    let bus = Arc::new(InProcessEventBus::new(16)) as Arc<dyn EventBus>;
    let mut rx = bus.subscribe();

    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(OkStep)
            .emit_lifecycle_events(true)
            .with_event_bus(Arc::clone(&bus)),
    );
    pipeline.run(&mut 0_i32).await.unwrap();

    let started = rx.recv().await.expect("StepStarted event");
    assert_eq!(started.event_type(), "pipeline.step_started");
    assert_eq!(started.aggregate_id(), "ok-step");

    let completed = rx.recv().await.expect("StepCompleted event");
    assert_eq!(completed.event_type(), "pipeline.step_completed");
    assert_eq!(completed.aggregate_id(), "ok-step");
}

// ── error path ────────────────────────────────────────────────────────────────

/// @covers: DefaultPipeline::run — emit_lifecycle_events = true, step errors
#[tokio::test]
async fn test_pipeline_emits_step_failed_when_step_errors_error() {
    let bus = Arc::new(InProcessEventBus::new(16)) as Arc<dyn EventBus>;
    let mut rx = bus.subscribe();

    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(ErrStep)
            .emit_lifecycle_events(true)
            .with_event_bus(Arc::clone(&bus)),
    );
    let _ = pipeline.run(&mut 0_i32).await;

    let started = rx.recv().await.expect("StepStarted event");
    assert_eq!(started.event_type(), "pipeline.step_started");
    assert_eq!(started.aggregate_id(), "err-step");

    let failed = rx.recv().await.expect("StepFailed event");
    assert_eq!(failed.event_type(), "pipeline.step_failed");
    assert_eq!(failed.aggregate_id(), "err-step");
}

// ── with_event_bus builder method ────────────────────────────────────────────

/// @covers: PipelineBuilder::with_event_bus — bus is stored in the builder
#[test]
fn test_with_event_bus_stores_bus_happy() {
    let bus = Arc::new(InProcessEventBus::new(4)) as Arc<dyn EventBus>;
    let initial_count = Arc::strong_count(&bus);
    let builder = PipelineBuilder::<i32>::new().with_event_bus(Arc::clone(&bus));
    assert_eq!(
        Arc::strong_count(&bus),
        initial_count + 1,
        "builder must retain the cloned Arc"
    );
    assert!(builder.event_bus.is_some());
}

/// @covers: PipelineBuilder::with_event_bus — second call overwrites the first
#[test]
fn test_with_event_bus_overwrites_previous_bus_error() {
    let bus1 = Arc::new(InProcessEventBus::new(4)) as Arc<dyn EventBus>;
    let bus2 = Arc::new(InProcessEventBus::new(8)) as Arc<dyn EventBus>;
    let count1_before = Arc::strong_count(&bus1);
    let count2_before = Arc::strong_count(&bus2);
    let builder = PipelineBuilder::<i32>::new()
        .with_event_bus(Arc::clone(&bus1))
        .with_event_bus(Arc::clone(&bus2));
    assert_eq!(Arc::strong_count(&bus1), count1_before, "first bus must be released");
    assert_eq!(Arc::strong_count(&bus2), count2_before + 1, "second bus must be retained");
    assert!(builder.event_bus.is_some());
}

/// @covers: PipelineBuilder::with_event_bus — default builder has no bus
#[test]
fn test_with_event_bus_absent_when_not_set_edge() {
    let builder = PipelineBuilder::<i32>::new();
    assert!(builder.event_bus.is_none());
}

// ── edge case ─────────────────────────────────────────────────────────────────

/// @covers: DefaultPipeline::run — emit_lifecycle_events = false, bus present but silent
#[tokio::test]
async fn test_pipeline_emits_no_events_when_flag_disabled_edge() {
    let bus = Arc::new(InProcessEventBus::new(16)) as Arc<dyn EventBus>;
    let mut rx = bus.subscribe();

    let pipeline = PipelineSvc::build(
        PipelineBuilder::new()
            .with(OkStep)
            .emit_lifecycle_events(false)
            .with_event_bus(Arc::clone(&bus)),
    );
    pipeline.run(&mut 0_i32).await.unwrap();

    // Bus must stay silent — timeout proves no event was published.
    let result = tokio::time::timeout(Duration::from_millis(50), rx.recv()).await;
    assert!(
        result.is_err(),
        "no events must be emitted when emit_lifecycle_events = false"
    );
}
