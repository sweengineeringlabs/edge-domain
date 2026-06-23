//! [`PipelineBuilder<Ctx>`] — fluent API for composing pipelines.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{Pipeline, PipelineConfig, Step};
use crate::spi::default_pipeline::DefaultPipeline;

/// Fluent builder for composing [`DefaultPipeline<Ctx>`] from steps.
///
/// This is an internal implementation detail. Use factory functions in the saf module.
///
/// # Example (internal use)
///
/// ```ignore
/// let pipeline = PipelineBuilder::new()
///     .with(ExtractTokenStep)
///     .with(VerifyTokenStep)
///     .with_timeout(Duration::from_secs(10))
///     .with(IdentifyCallerStep)
///     .build();
/// ```
pub(crate) struct PipelineBuilder<Ctx> {
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
}

impl<Ctx: Send + 'static> PipelineBuilder<Ctx> {
    /// Create a new empty builder.
    pub(crate) fn new() -> Self {
        Self {
            steps: vec![],
            config: PipelineConfig::default(),
        }
    }

    /// Add a step to the pipeline.
    ///
    /// Steps are executed in the order they are added.
    pub(crate) fn with<S: Step<Ctx> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Add a step conditionally.
    ///
    /// If `condition` is true, the step is added; otherwise it is skipped.
    pub(crate) fn with_if<S: Step<Ctx> + 'static>(self, condition: bool, step: S) -> Self {
        if condition {
            self.with(step)
        } else {
            self
        }
    }

    /// Set a per-step timeout.
    pub(crate) fn with_timeout(mut self, duration: Duration) -> Self {
        self.config.timeout_per_step = Some(duration);
        self
    }

    /// Enable lifecycle event emission.
    pub(crate) fn with_lifecycle_events(mut self, enabled: bool) -> Self {
        self.config.emit_lifecycle_events = enabled;
        self
    }

    /// Set whether to abort on error.
    pub(crate) fn abort_on_error(mut self, abort: bool) -> Self {
        self.config.abort_on_error = abort;
        self
    }

    /// Build the pipeline.
    pub(crate) fn build(self) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::with_config(self.steps, self.config))
    }
}

impl<Ctx: Send + 'static> Default for PipelineBuilder<Ctx> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for new constructor
    /// @covers: new
    #[test]
    fn test_new_happy_creates_empty_builder() {
        let builder = PipelineBuilder::<()>::new();
        assert_eq!(builder.build().step_count(), 0);
    }

    // Tests for with_timeout method
    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_happy_sets_duration() {
        let builder = PipelineBuilder::<()>::new().with_timeout(Duration::from_secs(5));
        let pipeline = builder.build();
        assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
    }

    // Tests for with_lifecycle_events method
    /// @covers: with_lifecycle_events
    #[test]
    fn test_with_lifecycle_events_happy_enables_events() {
        let builder = PipelineBuilder::<()>::new().with_lifecycle_events(true);
        let pipeline = builder.build();
        assert!(pipeline.config().emit_lifecycle_events);
    }

    // Tests for abort_on_error method
    /// @covers: abort_on_error
    #[test]
    fn test_abort_on_error_happy_disables_abort() {
        let builder = PipelineBuilder::<()>::new().abort_on_error(false);
        let pipeline = builder.build();
        assert!(!pipeline.config().abort_on_error);
    }

    // Tests for with_if method
    /// @covers: with_if
    #[test]
    fn test_with_if_happy_condition_true_adds_step() {
        struct DummyStep;
        #[async_trait::async_trait]
        impl Step<()> for DummyStep {
            async fn execute(&self, _ctx: &mut ()) -> Result<(), crate::api::PipelineError> {
                Ok(())
            }
            fn name(&self) -> &str {
                "dummy"
            }
        }

        let builder = PipelineBuilder::<()>::new().with_if(true, DummyStep);
        let pipeline = builder.build();
        assert_eq!(pipeline.step_count(), 1);
    }

    /// @covers: with_if
    #[test]
    fn test_with_if_happy_condition_false_skips_step() {
        struct DummyStep;
        #[async_trait::async_trait]
        impl Step<()> for DummyStep {
            async fn execute(&self, _ctx: &mut ()) -> Result<(), crate::api::PipelineError> {
                Ok(())
            }
            fn name(&self) -> &str {
                "dummy"
            }
        }

        let builder = PipelineBuilder::<()>::new().with_if(false, DummyStep);
        let pipeline = builder.build();
        assert_eq!(pipeline.step_count(), 0);
    }

    // Tests for build method
    /// @covers: build
    #[test]
    fn test_build_happy_creates_pipeline() {
        let pipeline = PipelineBuilder::<()>::new()
            .with_timeout(Duration::from_secs(10))
            .build();
        assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(10)));
    }
}
