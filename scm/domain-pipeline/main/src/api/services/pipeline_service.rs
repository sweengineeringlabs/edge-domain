//! Pipeline service factory — creates pipeline instances.

use std::sync::Arc;
use super::super::{Pipeline, PipelineConfig, Step};

/// Service for creating pipelines.
pub struct PipelineService;

impl PipelineService {
    /// Create a pipeline with the given steps and default config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create_pipeline<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
    ) -> Box<dyn Pipeline<Ctx>> {
        crate::spi::PipelineFactory::create(steps)
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create_pipeline_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        crate::spi::PipelineFactory::create_with_config(steps, config)
    }
}
