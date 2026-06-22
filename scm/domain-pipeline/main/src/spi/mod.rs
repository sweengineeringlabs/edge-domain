//! Strategy and builder implementations.

pub(crate) mod config_validator;
mod pipeline_builder;
mod noop_step;

pub use pipeline_builder::PipelineBuilder;
pub use noop_step::{NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};

/// Create a config validator strategy.
pub fn create_validator(enabled: bool) -> Box<dyn crate::api::Validator> {
    Box::new(config_validator::ConfigValidator::new(enabled))
}

/// Create a default pipeline (delegates to core).
pub fn create_default_pipeline<Ctx: Send + 'static>(
    steps: std::vec::Vec<std::sync::Arc<dyn crate::api::Step<Ctx>>>,
) -> Box<dyn crate::api::Pipeline<Ctx>> {
    Box::new(crate::core::default_pipeline::DefaultPipeline::new(steps))
}

/// Create a default pipeline with config (delegates to core).
pub fn create_default_pipeline_with_config<Ctx: Send + 'static>(
    steps: std::vec::Vec<std::sync::Arc<dyn crate::api::Step<Ctx>>>,
    config: crate::api::PipelineConfig,
) -> Box<dyn crate::api::Pipeline<Ctx>> {
    Box::new(crate::core::default_pipeline::DefaultPipeline::with_config(steps, config))
}
