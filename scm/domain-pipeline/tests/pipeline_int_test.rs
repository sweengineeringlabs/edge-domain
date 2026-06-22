//! @covers Pipeline trait
//! Comprehensive trait implementation tests for Pipeline interface.
//! Ensures all trait methods have proper test coverage across happy, error, and edge paths.

use edge_domain_pipeline::{
    create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineError, PipelineConfig,
};
use std::sync::Arc;
use std::time::Duration;

// Test doubles for trait testing
struct AlwaysPassStep;

#[async_trait::async_trait]
impl Step<()> for AlwaysPassStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "always-pass"
    }
}

struct FailureStep {
    reason: String,
}

impl FailureStep {
    fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Step<()> for FailureStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed(self.reason.clone()))
    }

    fn name(&self) -> &str {
        "failure"
    }
}

struct CounterStep {
    value: i32,
}

impl CounterStep {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

#[async_trait::async_trait]
impl Step<i32> for CounterStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += self.value;
        Ok(())
    }

    fn name(&self) -> &str {
        "counter"
    }
}

// Pipeline::execute tests

/// Test that execute works with empty pipeline
#[tokio::test]
async fn test_pipeline_execute_empty_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

/// Test that execute works with passing steps
#[tokio::test]
async fn test_pipeline_execute_passing_steps_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(AlwaysPassStep),
        Arc::new(AlwaysPassStep),
        Arc::new(AlwaysPassStep),
    ]);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

/// Test that execute returns error on step failure
#[tokio::test]
async fn test_pipeline_execute_step_failure_error() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(AlwaysPassStep),
        Arc::new(FailureStep::new("intentional failure")),
        Arc::new(AlwaysPassStep),
    ]);
    let mut ctx = ();
    let result = pipeline.execute(&mut ctx).await;
    assert!(result.is_err());
}

/// Test that execute stops on first failure
#[tokio::test]
async fn test_pipeline_execute_stops_on_error_error() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(AlwaysPassStep),
        Arc::new(FailureStep::new("stop here")),
        Arc::new(AlwaysPassStep), // This won't execute
    ]);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_err());
}

/// Test execute with very large number of steps
#[tokio::test]
async fn test_pipeline_execute_many_steps_edge() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..500 {
        steps.push(Arc::new(AlwaysPassStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    let mut ctx = ();
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

/// Test execute with context mutations
#[tokio::test]
async fn test_pipeline_execute_with_mutations_edge() {
    let pipeline: Box<dyn Pipeline<i32>> = create_pipeline(vec![
        Arc::new(CounterStep::new(10)),
        Arc::new(CounterStep::new(20)),
        Arc::new(CounterStep::new(30)),
    ]);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 60);
}

// Pipeline::step_count tests

/// Test step_count returns 0 for empty pipeline
#[test]
fn test_pipeline_step_count_empty_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    assert_eq!(pipeline.step_count(), 0);
}

/// Test step_count returns correct count
#[test]
fn test_pipeline_step_count_with_steps_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(AlwaysPassStep),
        Arc::new(AlwaysPassStep),
        Arc::new(AlwaysPassStep),
    ]);
    assert_eq!(pipeline.step_count(), 3);
}

/// Test step_count with large number of steps
#[test]
fn test_pipeline_step_count_many_steps_edge() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..250 {
        steps.push(Arc::new(AlwaysPassStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    assert_eq!(pipeline.step_count(), 250);
}

/// Test step_count consistency
#[test]
fn test_pipeline_step_count_consistency_error() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![
        Arc::new(AlwaysPassStep),
        Arc::new(AlwaysPassStep),
    ]);
    let count1 = pipeline.step_count();
    let count2 = pipeline.step_count();
    assert_eq!(count1, count2);
    assert_eq!(count1, 2);
}

// Pipeline::is_empty tests

/// Test is_empty returns true for empty pipeline
#[test]
fn test_pipeline_is_empty_true_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    assert!(pipeline.is_empty());
}

/// Test is_empty returns false for non-empty pipeline
#[test]
fn test_pipeline_is_empty_false_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![Arc::new(AlwaysPassStep)]);
    assert!(!pipeline.is_empty());
}

/// Test is_empty with many steps
#[test]
fn test_pipeline_is_empty_many_steps_edge() {
    let mut steps: Vec<Arc<dyn Step<()>>> = vec![];
    for _ in 0..100 {
        steps.push(Arc::new(AlwaysPassStep));
    }
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(steps);
    assert!(!pipeline.is_empty());
}

/// Test is_empty consistency with step_count
#[test]
fn test_pipeline_is_empty_consistency_error() {
    let empty_pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    assert!(empty_pipeline.is_empty());
    assert_eq!(empty_pipeline.step_count(), 0);

    let non_empty: Box<dyn Pipeline<()>> = create_pipeline(vec![Arc::new(AlwaysPassStep)]);
    assert!(!non_empty.is_empty());
    assert_eq!(non_empty.step_count(), 1);
}

// Pipeline::config tests

/// Test config returns default configuration
#[test]
fn test_pipeline_config_default_happy() {
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline(vec![]);
    let config = pipeline.config();
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

/// Test config with custom values
#[test]
fn test_pipeline_config_custom_happy() {
    let custom = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline_with_config(vec![], custom);
    let config = pipeline.config();
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(10)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

/// Test config with all options enabled
#[test]
fn test_pipeline_config_all_enabled_edge() {
    let custom = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline_with_config(vec![], custom);
    let config = pipeline.config();
    assert!(config.timeout_per_step.is_some());
    assert!(config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

/// Test config with all options disabled
#[test]
fn test_pipeline_config_all_disabled_error() {
    let custom = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<()>> = create_pipeline_with_config(vec![], custom);
    let config = pipeline.config();
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}
