//! Convenience free functions — the crate's public construction surface.
//!
//! Constructs concrete core implementations and returns opaque trait objects.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineBuilder, PipelineConfig, Step, Validator};
use crate::core::traits::DefaultValidator;
use crate::core::pipeline::DefaultPipeline;

/// Create a pipeline with default configuration.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::new(steps))
}

/// Create a pipeline with custom configuration.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(steps, config))
}

/// Create a validator with the given enabled state.
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    Box::new(DefaultValidator::new(enabled))
}

/// Build a pipeline from a completed [`PipelineBuilder`].
///
/// This is the terminal step of the builder pattern:
/// ```rust,ignore
/// let pipeline = build_pipeline(
///     PipelineBuilder::new()
///         .with(my_step)
///         .with_timeout(Duration::from_secs(5))
/// );
/// ```
pub fn build_pipeline<Ctx: Send + 'static>(builder: PipelineBuilder<Ctx>) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(builder.steps, builder.config))
}
