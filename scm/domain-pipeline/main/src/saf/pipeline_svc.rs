//! Pipeline service wrapper — factory and facade for creating pipelines.
//!
//! This module provides the implementation-facing factory for creating pipelines.
//! The public API is available through convenience functions in lib.rs.

use std::sync::Arc;
use crate::api::{Pipeline, PipelineConfig, PipelineService, Step};

/// Service marker constant for pipeline factory operations.
pub const PIPELINE_FACTORY: &str = "pipeline_factory";

/// Internal factory for creating pipeline instances.
///
/// This factory conceals the concrete implementation type, returning opaque trait objects.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PipelineFactory;

impl PipelineFactory {
    /// Create a pipeline with the given steps and default config.
    ///
    /// This is a convenience wrapper delegating to [`PipelineService`].
    pub(crate) fn create<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
    ) -> Box<dyn Pipeline<Ctx>> {
        PipelineService::create_pipeline(steps)
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// This is a convenience wrapper delegating to [`PipelineService`].
    pub(crate) fn create_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        PipelineService::create_pipeline_with_config(steps, config)
    }
}
