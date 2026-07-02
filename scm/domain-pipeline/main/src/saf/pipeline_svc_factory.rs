//! Pipeline service — opaque construction surface for [`Pipeline`](crate::api::Pipeline).

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use edge_domain_event::EventBus;

use crate::api::{Pipeline, PipelineBuilder, PipelineConfig, Step};
use crate::core::traits::DefaultPipeline;

/// Identifies the pipeline `Service` implementation at runtime.
pub const PIPELINE_SVC: &str = "pipeline";

/// Identifies the `PipelineSvc` factory implementation.
pub const PIPELINE_SVC_FACTORY: &str = "pipeline_svc_factory";

/// Construction handle for [`Pipeline`](crate::api::Pipeline) instances.
///
/// Consumers declare a dependency on `Box<dyn Pipeline<Ctx = Ctx, E = E, ..>>` (exclusive
/// ownership) or `Arc<dyn Pipeline<Ctx = Ctx, E = E, ..>>` (shared ownership). The concrete
/// implementation
/// (`DefaultPipeline`) is never exposed.
///
/// # Examples
///
/// ## Exclusive ownership
///
/// ```rust,ignore
/// use edge_domain_pipeline::{PipelineSvc, PipelineBuilder};
///
/// let pipeline = PipelineSvc::build(
///     PipelineBuilder::new()
///         .with(EnrichStep)
///         .abort_on_error(true),
/// );
/// pipeline.run(&mut ctx).await?;
/// ```
pub struct PipelineSvc;

impl PipelineSvc {
    /// Build a pipeline with exclusive ownership.
    pub fn build<Ctx, E>(
        builder: PipelineBuilder<Ctx, E>,
    ) -> Box<dyn Pipeline<Ctx = Ctx, E = E, Request = Ctx, Response = Ctx>>
    where
        Ctx: Send + 'static,
        E: fmt::Display + fmt::Debug + Send + 'static,
    {
        let pipeline = DefaultPipeline::with_config(builder.steps, builder.config);
        let pipeline = match builder.event_bus {
            Some(bus) => pipeline.with_event_bus(bus),
            None => pipeline,
        };
        Box::new(pipeline)
    }

    /// Build a pipeline with shared ownership.
    pub fn build_shared<Ctx, E>(
        builder: PipelineBuilder<Ctx, E>,
    ) -> Arc<dyn Pipeline<Ctx = Ctx, E = E, Request = Ctx, Response = Ctx>>
    where
        Ctx: Send + 'static,
        E: fmt::Display + fmt::Debug + Send + 'static,
    {
        let pipeline = DefaultPipeline::with_config(builder.steps, builder.config);
        let pipeline = match builder.event_bus {
            Some(bus) => pipeline.with_event_bus(bus),
            None => pipeline,
        };
        Arc::new(pipeline)
    }
}

impl<Ctx, E> PipelineBuilder<Ctx, E>
where
    Ctx: Send + 'static,
    E: Send + 'static,
{
    /// Create a new builder with default configuration and no steps.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
            event_bus: None,
        }
    }

    /// Append a step to the execution sequence.
    pub fn with<S: Step<Ctx = Ctx, ExecutionError = E> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Append a shared step (useful when the same step is reused across pipelines).
    pub fn with_shared(mut self, step: Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>) -> Self {
        self.steps.push(step);
        self
    }

    /// Apply a per-step execution timeout; steps that exceed it produce
    /// [`PipelineError::StepTimeout`](crate::PipelineError::StepTimeout).
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

    /// Attach an event bus for lifecycle event emission.
    ///
    /// Events are only published when [`emit_lifecycle_events`](Self::emit_lifecycle_events)
    /// is also set to `true`.
    pub fn with_event_bus(mut self, bus: Arc<dyn EventBus>) -> Self {
        self.event_bus = Some(bus);
        self
    }
}
