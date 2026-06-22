//! Integration tests for the [`PipelineBuilder`] API.
//!
//! @covers PipelineBuilder

use edge_domain_pipeline::{PipelineBuilder, Pipeline, Step, PipelineError, AlwaysPassStep, AlwaysFailStep, MutatingStep};
use std::time::Duration;

/// @covers: build
#[test]
fn test_spi_pipeline_builder_builds_pipeline() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with(AlwaysPassStep::new())
        .build();

    assert_eq!(pipeline.step_count(), 1);
}

/// @covers: with
#[test]
fn test_spi_pipeline_builder_multi_step() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with(AlwaysPassStep::new())
        .with(AlwaysPassStep::new())
        .with(AlwaysPassStep::new())
        .build();

    assert_eq!(pipeline.step_count(), 3);
}

/// @covers: with_if
#[test]
fn test_spi_pipeline_builder_with_if_condition() {
    let pipeline_true: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with_if(true, AlwaysPassStep::new())
        .build();

    let pipeline_false: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with_if(false, AlwaysPassStep::new())
        .build();

    assert_eq!(pipeline_true.step_count(), 1);
    assert_eq!(pipeline_false.step_count(), 0);
}

/// @covers: with_if
#[test]
fn test_spi_pipeline_builder_mixed_conditions() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with_if(true, AlwaysPassStep::new())
        .with_if(false, AlwaysPassStep::new())
        .with_if(true, AlwaysPassStep::new())
        .build();

    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: with_timeout
#[test]
fn test_spi_pipeline_builder_with_timeout_nominal_happy() {
    let timeout = Duration::from_secs(30);
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with_timeout(timeout)
        .build();

    assert_eq!(pipeline.config().timeout_per_step, Some(timeout));
}

/// @covers: with_lifecycle_events
#[test]
fn test_spi_pipeline_builder_with_lifecycle_events() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with_lifecycle_events(true)
        .build();

    assert!(pipeline.config().emit_lifecycle_events);
}

/// @covers: abort_on_error
#[test]
fn test_spi_pipeline_builder_abort_on_error_false() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .abort_on_error(false)
        .build();

    assert!(!pipeline.config().abort_on_error);
}

/// @covers: build
#[test]
fn test_spi_pipeline_builder_chaining_all_options() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::new()
        .with(AlwaysPassStep::new())
        .with_timeout(Duration::from_secs(5))
        .with(AlwaysPassStep::new())
        .with_lifecycle_events(true)
        .with(AlwaysFailStep::new("expected"))
        .abort_on_error(false)
        .build();

    assert_eq!(pipeline.step_count(), 3);
    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
    assert!(pipeline.config().emit_lifecycle_events);
    assert!(!pipeline.config().abort_on_error);
}

#[tokio::test]
async fn test_spi_pipeline_builder_executes_pipeline() {
    let pipeline = PipelineBuilder::new()
        .with(MutatingStep::new(|ctx: &mut i32| *ctx += 10))
        .with(MutatingStep::new(|ctx: &mut i32| *ctx *= 2))
        .build();

    let mut ctx = 5;
    assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    assert_eq!(ctx, 30);
}

#[tokio::test]
async fn test_spi_pipeline_builder_with_fail_step() {
    let pipeline = PipelineBuilder::new()
        .with(AlwaysPassStep::new())
        .with(AlwaysFailStep::new("boom"))
        .with(AlwaysPassStep::new())
        .build();

    let mut ctx = 0i32;
    let result = Pipeline::execute(&pipeline, &mut ctx).await;
    assert!(result.is_err());
}

#[test]
fn test_spi_pipeline_builder_default_is_empty() {
    let pipeline: impl edge_domain_pipeline::Pipeline = PipelineBuilder::default().build();
    assert!(pipeline.is_empty());
}
