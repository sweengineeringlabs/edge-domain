//! Pipeline service facade — provides orchestration interface.

use std::sync::Arc;
use crate::api::{Step, PipelineError};
use crate::spi::{create_default_pipeline, create_default_pipeline_with_config};

// Re-export public types from api (through this module)
pub use crate::api::{Pipeline, PipelineConfig};

/// Marker constant for pipeline service identification.
pub const PIPELINE_SVC: &str = "pipeline";

/// Create a pipeline with given steps and default config.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn crate::api::Pipeline<Ctx>> {
    create_default_pipeline(steps)
}

/// Create a pipeline with custom config.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn crate::api::Pipeline<Ctx>> {
    create_default_pipeline_with_config(steps, config)
}

/// Adapter wrapper that allows a Pipeline to be used as a Step.
/// This is useful for nested pipelines and composition.
pub struct PipelineAsStep<Ctx: Send + 'static> {
    pipeline: Box<dyn Pipeline<Ctx>>,
}

impl<Ctx: Send + 'static> PipelineAsStep<Ctx> {
    /// Wrap a pipeline so it can be used as a step.
    pub fn new(pipeline: Box<dyn Pipeline<Ctx>>) -> Self {
        Self { pipeline }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send + 'static> Step<Ctx> for PipelineAsStep<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        self.pipeline.execute(ctx).await
    }

    fn name(&self) -> &str {
        "pipeline-as-step"
    }
}
