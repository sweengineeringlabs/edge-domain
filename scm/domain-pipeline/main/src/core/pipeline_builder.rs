//! [`PipelineBuilder<Ctx>`] — fluent API for composing pipelines.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{Pipeline, PipelineConfig, Step};
use super::default_pipeline::DefaultPipeline;

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
    pub fn build(self) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::with_config(self.steps, self.config))
    }
}

impl<Ctx: Send + 'static> Default for PipelineBuilder<Ctx> {
    fn default() -> Self {
        Self::new()
    }
}
