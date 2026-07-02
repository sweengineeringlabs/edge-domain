//! Integration tests for [`ParallelStepBuilder`] and [`ParallelConfig`] — RFC-002.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use edge_domain_pipeline::{
    ContextMutationRequest, ParallelConfig, ParallelStepBuilder, ParallelStepSvc, Step,
    StepNameRequest, StepNameResponse,
};

struct NoopStep;

#[async_trait::async_trait]
impl Step for NoopStep {
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
            name: "noop".to_string(),
        })
    }
}

// ── ParallelStepBuilder::new ────────────────────────────────────────────────────

#[test]
fn test_new_happy_creates_empty_builder() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert!(builder.steps.is_empty());
    assert!(!builder.config.fail_fast, "fail_fast must default to false");
}

#[test]
fn test_new_error_default_config_has_no_timeout() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert!(
        builder.config.timeout_per_branch.is_none(),
        "default has no per-branch timeout"
    );
}

#[test]
fn test_new_edge_two_calls_produce_independent_builders() {
    let b1: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    let b2: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert_eq!(b1.steps.len(), b2.steps.len());
    assert_eq!(b1.config.fail_fast, b2.config.fail_fast);
}

// ── Default impl ─────────────────────────────────────────────────────────────────

#[test]
fn test_default_happy_matches_new() {
    let via_default: ParallelStepBuilder<i32, String> = ParallelStepBuilder::default();
    let via_new: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert_eq!(via_default.steps.len(), via_new.steps.len());
    assert_eq!(via_default.config.fail_fast, via_new.config.fail_fast);
    assert_eq!(
        via_default.config.timeout_per_branch,
        via_new.config.timeout_per_branch
    );
}

// ── Chained configuration ─────────────────────────────────────────────────────────

#[test]
fn test_builder_happy_chained_configuration() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new()
        .timeout_per_branch(Duration::from_secs(5))
        .fail_fast(true)
        .emit_lifecycle_events(true);
    assert_eq!(
        builder.config.timeout_per_branch,
        Some(Duration::from_secs(5))
    );
    assert!(builder.config.fail_fast);
    assert!(builder.config.emit_lifecycle_events);
}

#[test]
fn test_builder_error_fail_fast_defaults_false_until_set() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert!(
        !builder.config.fail_fast,
        "collecting all failures must be the default, not fail_fast"
    );
}

#[test]
fn test_builder_edge_repeated_calls_use_last_value() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new()
        .fail_fast(true)
        .fail_fast(false)
        .fail_fast(true);
    assert!(builder.config.fail_fast, "last call must win");
}

// ── with / with_shared ────────────────────────────────────────────────────────────

#[test]
fn test_with_happy_appends_step() {
    let builder: ParallelStepBuilder<i32, String> =
        ParallelStepBuilder::new().with(NoopStep).with(NoopStep);
    assert_eq!(builder.steps.len(), 2);
}

#[tokio::test]
async fn test_with_shared_happy_reuses_step() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::new(NoopStep);
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new()
        .with_shared(step.clone())
        .with_shared(step);
    assert_eq!(builder.steps.len(), 2);
    let parallel = ParallelStepSvc::build(builder);
    let mut ctx = 0;
    assert!(parallel
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[test]
fn test_with_edge_empty_builder_has_no_steps() {
    let builder: ParallelStepBuilder<i32, String> = ParallelStepBuilder::new();
    assert!(builder.steps.is_empty());
}

// ── ParallelConfig ─────────────────────────────────────────────────────────────────

#[test]
fn test_parallel_config_happy_default_values() {
    let config = ParallelConfig::default();
    assert!(config.timeout_per_branch.is_none());
    assert!(!config.fail_fast);
    assert!(!config.emit_lifecycle_events);
}

#[test]
fn test_parallel_config_error_fail_fast_true() {
    let config = ParallelConfig {
        fail_fast: true,
        ..ParallelConfig::default()
    };
    assert!(config.fail_fast);
}

#[test]
fn test_parallel_config_edge_all_options_set() {
    let config = ParallelConfig {
        timeout_per_branch: Some(Duration::from_millis(50)),
        fail_fast: true,
        emit_lifecycle_events: true,
    };
    assert!(config.timeout_per_branch.is_some());
    assert!(config.fail_fast);
    assert!(config.emit_lifecycle_events);
}
