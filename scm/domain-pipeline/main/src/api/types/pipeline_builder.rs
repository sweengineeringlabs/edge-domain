//! [`PipelineBuilder`] — fluent builder for assembling a pipeline from steps and configuration.

use std::sync::Arc;

use edge_domain_event::EventBus;

use crate::api::{PipelineConfig, Step};

/// Fluent builder for assembling a pipeline.
///
/// Accepts steps one at a time (or shared arcs) and accumulates configuration.
/// Hand the completed builder to [`PipelineSvc::build`](crate::PipelineSvc::build) to
/// construct the concrete pipeline.
pub struct PipelineBuilder<Ctx> {
    /// Ordered list of steps to execute.
    pub steps: Vec<Arc<dyn Step<Ctx>>>,
    /// Accumulated pipeline configuration.
    pub config: PipelineConfig,
    /// Optional event bus for lifecycle event emission.
    pub event_bus: Option<Arc<dyn EventBus>>,
}
