//! Service Abstraction Framework — re-exports and service name constants.
//!
//! Provides trait re-exports and constants for service access.

pub mod pipeline_svc;
pub mod step_svc;
pub mod validator_svc;

/// Service name constant for pipeline service.
pub const PIPELINE_SVC: &str = "pipeline";

/// Service name constant for step service.
pub const STEP_SVC: &str = "step";

/// Service name constant for validator service.
pub const VALIDATOR_SVC: &str = "validator";

/// Create a pipeline with the given steps and default config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
///
/// This is a convenience wrapper around [`pipeline_svc`].
pub use pipeline_svc::create as create_pipeline;

/// Create a pipeline with the given steps and custom config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
///
/// This is a convenience wrapper around [`pipeline_svc`].
pub use pipeline_svc::create_with_config as create_pipeline_with_config;

/// Create a config validator strategy.
///
/// This is a convenience wrapper around [`validator_svc`].
pub use validator_svc::create as create_validator;
