//! [`PipelineError<E>`] — error type for pipeline execution.

use super::step_error::StepError;

/// Errors that can occur during pipeline execution.
///
/// `E` is the consumer's domain error type (e.g. `SecurityError`). The engine
/// wraps it in [`StepFailed`](PipelineError::StepFailed) with the step name added
/// as context. Engine-level faults (`StepTimeout`, `ConfigError`, `UnknownStep`) do
/// not carry `E`.
#[non_exhaustive]
#[derive(Debug)]
pub enum PipelineError<E> {
    /// A step returned `Err(cause)`. The engine adds `step_name` context.
    StepFailed(StepError<E>),

    /// A step exceeded its configured per-step timeout.
    StepTimeout {
        /// Name of the step that timed out.
        step_name: String,
    },

    /// Invalid pipeline configuration (startup-time).
    ConfigError(String),

    /// A step name in a [`PipelineDefinition`](crate::PipelineDefinition) was not found
    /// in the registry (startup-time).
    UnknownStep(String),
}
