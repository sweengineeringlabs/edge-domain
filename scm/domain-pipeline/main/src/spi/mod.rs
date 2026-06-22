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
