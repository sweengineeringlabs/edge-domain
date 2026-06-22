//! [`PipelineBuilder<Ctx>`] — fluent API for composing pipelines.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{PipelineConfig, Step};
use crate::core::default_pipeline::DefaultPipeline;

/// Fluent builder for composing [`DefaultPipeline<Ctx>`] from steps.
///
/// # Example
///
/// ```ignore
/// let pipeline = PipelineBuilder::new()
///     .with(ExtractTokenStep)
///     .with(VerifyTokenStep)
///     .with_timeout(Duration::from_secs(10))
///     .with(IdentifyCallerStep)
///     .build();
/// ```
pub struct PipelineBuilder<Ctx> {
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
}

impl<Ctx: Send + 'static> PipelineBuilder<Ctx> {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            steps: vec![],
            config: PipelineConfig::default(),
        }
    }

    /// Add a step to the pipeline.
    ///
    /// Steps are executed in the order they are added.
    pub fn with<S: Step<Ctx> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Add a step conditionally.
    ///
    /// If `condition` is true, the step is added; otherwise it is skipped.
    pub fn with_if<S: Step<Ctx> + 'static>(self, condition: bool, step: S) -> Self {
        if condition {
            self.with(step)
        } else {
            self
        }
    }

    /// Set a per-step timeout.
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.config.timeout_per_step = Some(duration);
        self
    }

    /// Enable lifecycle event emission.
    pub fn with_lifecycle_events(mut self, enabled: bool) -> Self {
        self.config.emit_lifecycle_events = enabled;
        self
    }

    /// Set whether to abort on error.
    pub fn abort_on_error(mut self, abort: bool) -> Self {
        self.config.abort_on_error = abort;
        self
    }

    /// Build the pipeline.
    pub fn build(self) -> DefaultPipeline<Ctx> {
        DefaultPipeline::with_config(self.steps, self.config)
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
    use crate::api::Pipeline;

    struct DummyStep;

    #[async_trait::async_trait]
    impl Step<i32> for DummyStep {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), crate::api::PipelineError> {
            Ok(())
        }

        fn name(&self) -> &str {
            "dummy"
        }
    }

    #[test]
    fn test_builder_new() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new().build();
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_builder_with_step() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with(DummyStep)
            .build();
        assert_eq!(pipeline.step_count(), 1);
    }

    #[test]
    fn test_builder_with_multiple_steps() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with(DummyStep)
            .with(DummyStep)
            .with(DummyStep)
            .build();
        assert_eq!(pipeline.step_count(), 3);
    }

    #[test]
    fn test_builder_with_if_true() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with_if(true, DummyStep)
            .build();
        assert_eq!(pipeline.step_count(), 1);
    }

    #[test]
    fn test_builder_with_if_false() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with_if(false, DummyStep)
            .build();
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_builder_with_timeout() {
        let duration = Duration::from_secs(10);
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with_timeout(duration)
            .build();
        assert_eq!(pipeline.config().timeout_per_step, Some(duration));
    }

    #[test]
    fn test_builder_with_lifecycle_events() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with_lifecycle_events(true)
            .build();
        assert!(pipeline.config().emit_lifecycle_events);
    }

    #[test]
    fn test_builder_abort_on_error() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .abort_on_error(false)
            .build();
        assert!(!pipeline.config().abort_on_error);
    }

    #[test]
    fn test_builder_default() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::default().build();
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_builder_chaining() {
        let pipeline: DefaultPipeline<i32> = PipelineBuilder::new()
            .with(DummyStep)
            .with_timeout(Duration::from_secs(5))
            .with_lifecycle_events(true)
            .with(DummyStep)
            .abort_on_error(false)
            .build();

        assert_eq!(pipeline.step_count(), 2);
        assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
        assert!(pipeline.config().emit_lifecycle_events);
        assert!(!pipeline.config().abort_on_error);
    }
}
