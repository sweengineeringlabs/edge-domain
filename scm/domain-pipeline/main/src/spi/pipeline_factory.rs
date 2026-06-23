//! Factory for creating pipeline instances (implementation).

use std::sync::Arc;

use crate::api::{PipelineConfig, Pipeline, Step, PipelineFactory};
use crate::spi::default_pipeline::DefaultPipeline;

impl PipelineFactory {
    /// Create a pipeline with the given steps and default config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create<Ctx: Send + 'static>(steps: Vec<Arc<dyn Step<Ctx>>>) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::new(steps))
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::with_config(steps, config))
    }
}

