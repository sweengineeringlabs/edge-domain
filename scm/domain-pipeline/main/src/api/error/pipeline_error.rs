//! [`PipelineError`] — error type for pipeline execution.

use thiserror::Error;

/// Errors that can occur during pipeline execution.
#[derive(Debug, Error, Clone)]
pub enum PipelineError {
    /// A step returned an error.
    #[error("step failed: {0}")]
    StepFailed(String),

    /// A step exceeded its configured timeout.
    #[error("step timeout exceeded")]
    StepTimeout,

    /// Pipeline configuration error.
    #[error("configuration error: {0}")]
    ConfigError(String),
}
