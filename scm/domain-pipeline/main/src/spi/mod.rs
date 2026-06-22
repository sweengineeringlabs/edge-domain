//! Strategy and builder implementations (internal only).

pub(crate) mod config_validator;
pub(crate) mod noop;

// Re-export implementation types for use in saf/ (SAF layer accesses implementations via spi)
pub(crate) use crate::core::default::pipeline::DefaultPipeline;
pub(crate) use config_validator::ConfigValidator;
