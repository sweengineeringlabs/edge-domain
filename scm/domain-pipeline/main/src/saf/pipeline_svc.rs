//! Pipeline service facade — provides orchestration interface.

use std::sync::Arc;
use crate::api::Step;
use crate::core::default_pipeline::DefaultPipeline;

// Re-export public types from api
pub use crate::api::{Pipeline, PipelineConfig, PipelineError};

/// Marker constant for pipeline service identification.
pub const PIPELINE_SVC: &str = "pipeline";

/// Create a pipeline with given steps and default config.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::new(steps))
}

/// Create a pipeline with custom config.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(steps, config))
}
