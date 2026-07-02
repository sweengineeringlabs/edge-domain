//! [`PipelineBuilder<Ctx, E>`] — fluent builder for assembling a pipeline from steps and config.

use std::sync::Arc;

use edge_domain_event::EventBus;

use crate::api::{PipelineConfig, Step};

/// Fluent builder for assembling a pipeline.
///
/// `E` is the consumer's domain error type. All steps added via [`with`](Self::with) or
/// [`with_shared`](Self::with_shared) must implement `Step` with `Ctx = Ctx` and
/// `ExecutionError = E`.
///
/// Hand the completed builder to [`PipelineSvc::build`](crate::PipelineSvc::build) to
/// construct the concrete pipeline.
pub struct PipelineBuilder<Ctx, E> {
    /// Ordered list of steps to execute.
    pub steps: Vec<Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>>,
    /// Accumulated pipeline configuration.
    pub config: PipelineConfig,
    /// Optional event bus for lifecycle event emission.
    pub event_bus: Option<Arc<dyn EventBus>>,
}
