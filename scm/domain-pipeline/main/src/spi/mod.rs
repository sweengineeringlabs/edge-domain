//! Strategy and builder implementations (internal only).

pub(crate) mod builder;
pub(crate) mod config_validator;
pub(crate) mod default_pipeline;
pub(crate) mod dummy_step;
pub(crate) mod noop;
pub(crate) mod pipeline_factory;
pub(crate) mod validator_factory;

// Internal factory types - not exposed as public API
pub(crate) struct PipelineFactory;
pub(crate) struct ValidatorFactory;
