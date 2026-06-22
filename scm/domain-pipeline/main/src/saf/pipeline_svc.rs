//! Pipeline service facade — provides orchestration interface.

use std::sync::Arc;
use crate::api::Step;
use crate::spi::{create_default_pipeline, create_default_pipeline_with_config};

// Re-export public types from api (through this module)
pub use crate::api::{Pipeline, PipelineConfig, PipelineError};

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
