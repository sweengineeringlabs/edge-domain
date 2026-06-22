//! Service Abstraction Framework — public API surface (traits only).

mod pipeline_svc;
mod step_svc;
mod validator_svc;

// Re-export through _svc modules
pub use pipeline_svc::{create_pipeline, create_pipeline_with_config, Pipeline, PIPELINE_SVC};
pub use step_svc::{Step, STEP_SVC};
pub use validator_svc::{create_validator, Validator, VALIDATOR_SVC};
