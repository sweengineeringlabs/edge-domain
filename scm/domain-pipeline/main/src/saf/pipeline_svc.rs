//! Pipeline service wrapper — re-exports pipeline service.
//!
//! This module provides facade for creating pipelines through the service layer.

use std::sync::Arc;
use crate::api::{Pipeline, PipelineConfig, PipelineService, Step};

/// Factory for creating pipeline instances.
///
/// This facade provides convenient factory methods for constructing pipelines.
#[derive(Debug, Clone, Copy)]
pub struct PipelineFactory;

impl PipelineFactory {
    /// Create a pipeline with the given steps and default config.
    ///
    /// This is a convenience wrapper delegating to [`PipelineService`].
    pub fn create<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
    ) -> Box<dyn Pipeline<Ctx>> {
        PipelineService::create_pipeline(steps)
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// This is a convenience wrapper delegating to [`PipelineService`].
    pub fn create_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        PipelineService::create_pipeline_with_config(steps, config)
    }
}
