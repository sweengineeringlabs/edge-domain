//! Service Abstraction Framework — public API surface (traits, facades, and test utilities).

mod pipeline_svc;
mod step_svc;
mod validator_svc;

// Re-export through _svc modules (traits and factories only)
pub use pipeline_svc::{create_pipeline, create_pipeline_with_config, Pipeline, PipelineAsStep, PIPELINE_SVC};
pub use step_svc::{Step, STEP_SVC};
pub use validator_svc::{create_validator, Validator, VALIDATOR_SVC};

// Re-export error and config types from api layer through pipeline_svc
pub use pipeline_svc::{SvcPipelineError as PipelineError, SvcPipelineConfig as PipelineConfig};

// Test utility implementations (for integration tests and examples).
// These are convenience helpers for testing and should not be relied upon in production code.
pub use crate::spi::noop_step::{AlwaysPassStep, AlwaysFailStep, MutatingStep, NoopStep};
