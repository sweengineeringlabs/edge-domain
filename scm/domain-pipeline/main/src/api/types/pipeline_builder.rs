//! [`PipelineBuilder`] — fluent builder for assembling a pipeline from steps and configuration.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{PipelineConfig, Step};

/// Fluent builder for assembling a pipeline.
///
/// Accepts steps one at a time (or shared arcs) and accumulates configuration.
/// Hand the completed builder to [`build_pipeline`](crate::build_pipeline) to
/// construct the concrete pipeline.
pub struct PipelineBuilder<Ctx> {
    /// Ordered list of steps to execute.
    pub steps: Vec<Arc<dyn Step<Ctx>>>,
    /// Accumulated pipeline configuration.
    pub config: PipelineConfig,
}

impl<Ctx: Send + 'static> PipelineBuilder<Ctx> {
    /// Create a new builder with default configuration and no steps.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
        }
    }

    /// Append a step to the execution sequence.
    pub fn with<S: Step<Ctx> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Append a shared step (useful when the same step is reused across pipelines).
    pub fn with_shared(mut self, step: Arc<dyn Step<Ctx>>) -> Self {
        self.steps.push(step);
        self
    }

    /// Apply a per-step execution timeout; steps that exceed it produce [`PipelineError::StepTimeout`](crate::PipelineError::StepTimeout).
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.config.timeout_per_step = Some(duration);
        self
    }

    /// Set whether the pipeline halts immediately on the first step error.
    ///
    /// Defaults to `true`. When set to `false` the pipeline continues
    /// past errors and returns `Ok(())` after all steps have run.
    pub fn abort_on_error(mut self, abort: bool) -> Self {
        self.config.abort_on_error = abort;
        self
    }

    /// Enable or disable lifecycle event emission.
    pub fn emit_lifecycle_events(mut self, emit: bool) -> Self {
        self.config.emit_lifecycle_events = emit;
        self
    }
}

impl<Ctx: Send + 'static> Default for PipelineBuilder<Ctx> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use crate::api::{PipelineError, Step};

    struct NoopStep;

    #[async_trait::async_trait]
    impl Step<i32> for NoopStep {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
            Ok(())
        }

        fn name(&self) -> &str {
            let n = "noop";
            n
        }
    }

    /// @covers: with
    #[test]
    fn test_with_happy_adds_step() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().with(NoopStep);
        assert_eq!(builder.steps.len(), 1);
    }

    /// @covers: with
    #[test]
    fn test_with_error_multiple_steps_accumulate() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .with(NoopStep)
            .with(NoopStep)
            .with(NoopStep);
        assert_eq!(builder.steps.len(), 3);
    }

    /// @covers: with
    #[test]
    fn test_with_edge_empty_builder_then_step() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new();
        assert!(builder.steps.is_empty());
        let builder = builder.with(NoopStep);
        assert_eq!(builder.steps.len(), 1);
    }

    /// @covers: with_shared
    #[test]
    fn test_with_shared_happy_adds_arc_step() {
        let step = Arc::new(NoopStep);
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().with_shared(step);
        assert_eq!(builder.steps.len(), 1);
    }

    /// @covers: with_shared
    #[test]
    fn test_with_shared_error_multiple_arcs_accumulate() {
        let s1 = Arc::new(NoopStep);
        let s2 = Arc::new(NoopStep);
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .with_shared(s1)
            .with_shared(s2);
        assert_eq!(builder.steps.len(), 2);
    }

    /// @covers: with_shared
    #[test]
    fn test_with_shared_edge_same_arc_twice() {
        let step: Arc<dyn Step<i32>> = Arc::new(NoopStep);
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .with_shared(step.clone())
            .with_shared(step);
        assert_eq!(builder.steps.len(), 2);
    }

    /// @covers: new
    #[test]
    fn test_new_happy_starts_empty() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new();
        assert!(builder.steps.is_empty());
        assert!(builder.config.timeout_per_step.is_none());
        assert!(builder.config.abort_on_error);
        assert!(!builder.config.emit_lifecycle_events);
    }

    /// @covers: new
    #[test]
    fn test_new_error_no_steps_has_default_config() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new();
        assert_eq!(builder.config.abort_on_error, PipelineConfig::default().abort_on_error);
    }

    /// @covers: new
    #[test]
    fn test_new_edge_default_impl_matches_new() {
        let a: PipelineBuilder<i32> = PipelineBuilder::new();
        let b: PipelineBuilder<i32> = PipelineBuilder::default();
        assert_eq!(a.config.abort_on_error, b.config.abort_on_error);
        assert_eq!(a.steps.len(), b.steps.len());
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_happy_sets_duration() {
        let dur = Duration::from_secs(5);
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().with_timeout(dur);
        assert_eq!(builder.config.timeout_per_step, Some(dur));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_error_zero_duration_stored() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().with_timeout(Duration::ZERO);
        assert_eq!(builder.config.timeout_per_step, Some(Duration::ZERO));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_edge_overrides_previous() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .with_timeout(Duration::from_secs(1))
            .with_timeout(Duration::from_secs(10));
        assert_eq!(builder.config.timeout_per_step, Some(Duration::from_secs(10)));
    }

    /// @covers: abort_on_error
    #[test]
    fn test_abort_on_error_happy_disable() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().abort_on_error(false);
        assert!(!builder.config.abort_on_error);
    }

    /// @covers: abort_on_error
    #[test]
    fn test_abort_on_error_error_re_enable() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .abort_on_error(false)
            .abort_on_error(true);
        assert!(builder.config.abort_on_error);
    }

    /// @covers: abort_on_error
    #[test]
    fn test_abort_on_error_edge_default_is_true() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new();
        assert!(builder.config.abort_on_error);
    }

    /// @covers: emit_lifecycle_events
    #[test]
    fn test_emit_lifecycle_events_happy_enable() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new().emit_lifecycle_events(true);
        assert!(builder.config.emit_lifecycle_events);
    }

    /// @covers: emit_lifecycle_events
    #[test]
    fn test_emit_lifecycle_events_error_disable() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new()
            .emit_lifecycle_events(true)
            .emit_lifecycle_events(false);
        assert!(!builder.config.emit_lifecycle_events);
    }

    /// @covers: emit_lifecycle_events
    #[test]
    fn test_emit_lifecycle_events_edge_default_is_false() {
        let builder: PipelineBuilder<i32> = PipelineBuilder::new();
        assert!(!builder.config.emit_lifecycle_events);
    }
}
