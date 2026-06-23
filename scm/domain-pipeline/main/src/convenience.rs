//! Convenience re-exports for direct function calls.

use std::sync::Arc;
use crate::api::{Pipeline, PipelineConfig, PipelineService, Step, Validator, ValidatorService};

/// Create a pipeline with default configuration.
///
/// Convenience function equivalent to `PipelineService::create_pipeline`.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    PipelineService::create_pipeline(steps)
}

/// Create a pipeline with custom configuration.
///
/// Convenience function equivalent to `PipelineService::create_pipeline_with_config`.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    PipelineService::create_pipeline_with_config(steps, config)
}

/// Create a validator with the given enabled state.
///
/// Convenience function equivalent to `ValidatorService::create_validator`.
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    ValidatorService::create_validator(enabled)
}
