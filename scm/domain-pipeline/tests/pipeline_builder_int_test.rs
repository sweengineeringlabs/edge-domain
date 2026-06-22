//! Integration tests for PipelineBuilder.

use edge_domain_pipeline::{Step, PipelineError, PipelineBuilder};
use std::time::Duration;

struct PassStep;

#[async_trait::async_trait]
impl Step<()> for PassStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "pass"
    }
}

struct CountingStep;

#[async_trait::async_trait]
impl Step<i32> for CountingStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        "counter"
    }
}

// Test with method
/// @covers: with
#[test]
fn test_pipeline_builder_with_happy_single_step() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with(CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 1);
}

/// @covers: with
#[test]
fn test_pipeline_builder_with_happy_multiple_steps() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with(CountingStep)
        .with(CountingStep)
        .with(CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 3);
}

/// @covers: with
#[test]
fn test_pipeline_builder_with_edge_chain_order() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with(CountingStep)
        .with(CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 2);
}

// Test with_if method
/// @covers: with_if
#[test]
fn test_pipeline_builder_with_if_happy_true_condition() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_if(true, CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 1);
}

/// @covers: with_if
#[test]
fn test_pipeline_builder_with_if_happy_false_condition() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_if(false, CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 0);
}

/// @covers: with_if
#[test]
fn test_pipeline_builder_with_if_edge_mixed_conditions() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_if(true, CountingStep)
        .with_if(false, CountingStep)
        .with_if(true, CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 2);
}

// Test with_timeout method
/// @covers: with_timeout
#[test]
fn test_pipeline_builder_with_timeout_happy_some() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_timeout(Duration::from_secs(10))
        .build();
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
}

/// @covers: with_timeout
#[test]
fn test_pipeline_builder_with_timeout_happy_zero() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_timeout(Duration::from_secs(0))
        .build();
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(0)));
}

/// @covers: with_timeout
#[test]
fn test_pipeline_builder_with_timeout_edge_large_duration() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_timeout(Duration::from_secs(3600))
        .build();
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(3600)));
}

// Test with_lifecycle_events method
/// @covers: with_lifecycle_events
#[test]
fn test_pipeline_builder_with_lifecycle_events_happy_enabled() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_lifecycle_events(true)
        .build();
    assert!(pipeline.config().emit_lifecycle_events);
}

/// @covers: with_lifecycle_events
#[test]
fn test_pipeline_builder_with_lifecycle_events_happy_disabled() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_lifecycle_events(false)
        .build();
    assert!(!pipeline.config().emit_lifecycle_events);
}

/// @covers: with_lifecycle_events
#[test]
fn test_pipeline_builder_with_lifecycle_events_edge_toggle() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_lifecycle_events(true)
        .with_lifecycle_events(false)
        .build();
    assert!(!pipeline.config().emit_lifecycle_events);
}

// Test abort_on_error method
/// @covers: abort_on_error
#[test]
fn test_pipeline_builder_abort_on_error_happy_enabled() {
    let pipeline = PipelineBuilder::<i32>::new()
        .abort_on_error(true)
        .build();
    assert!(pipeline.config().abort_on_error);
}

/// @covers: abort_on_error
#[test]
fn test_pipeline_builder_abort_on_error_happy_disabled() {
    let pipeline = PipelineBuilder::<i32>::new()
        .abort_on_error(false)
        .build();
    assert!(!pipeline.config().abort_on_error);
}

/// @covers: abort_on_error
#[test]
fn test_pipeline_builder_abort_on_error_edge_flip() {
    let pipeline = PipelineBuilder::<i32>::new()
        .abort_on_error(false)
        .abort_on_error(true)
        .build();
    assert!(pipeline.config().abort_on_error);
}

// Test build method
/// @covers: build
#[test]
fn test_pipeline_builder_build_happy_empty() {
    let pipeline = PipelineBuilder::<()>::new().build();
    assert_eq!(pipeline.step_count(), 0);
    assert!(pipeline.is_empty());
}

/// @covers: build
#[test]
fn test_pipeline_builder_build_happy_with_steps() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with(CountingStep)
        .with(CountingStep)
        .build();
    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: build
#[test]
fn test_pipeline_builder_build_edge_preserves_config() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with_timeout(Duration::from_secs(5))
        .with_lifecycle_events(true)
        .build();
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
    assert!(pipeline.config().emit_lifecycle_events);
}

// Fluent interface integration
/// @covers: build
#[test]
fn test_pipeline_builder_fluent_integration() {
    let pipeline = PipelineBuilder::<i32>::new()
        .with(CountingStep)
        .with_if(true, CountingStep)
        .with_timeout(Duration::from_secs(10))
        .with_lifecycle_events(true)
        .abort_on_error(false)
        .build();

    assert_eq!(pipeline.step_count(), 2);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}
