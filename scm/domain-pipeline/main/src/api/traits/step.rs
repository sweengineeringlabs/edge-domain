//! [`Step<Ctx>`] — single composable operation in a pipeline.

use super::super::error::PipelineError;

/// A single composable step in a [`Pipeline`](super::pipeline::Pipeline).
///
/// Steps execute sequentially, each receiving a mutable reference to shared context.
/// A step either succeeds (mutating context), fails (aborting the pipeline), or times out.
///
/// # Invariant
///
/// A step must be idempotent over the context — calling it twice with the same context
/// must produce the same result or error.
///
/// # Error Semantics
///
/// - `Ok(())` — Step succeeded; context mutations applied; continue to next step
/// - `Err(PipelineError::StepFailed(_))` — Step failed; abort pipeline
/// - `Err(PipelineError::StepTimeout)` — Step exceeded timeout; abort or skip based on config
/// - `Err(PipelineError::ConfigError(_))` — Configuration error; abort pipeline
pub trait Step<Ctx>: Send + Sync {
    /// Execute this step, mutating the context in-place.
    ///
    /// # Errors
    ///
    /// Returns [`PipelineError`] if the step fails. The context may be partially
    /// mutated before the error; callers must not assume rollback.
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;

    /// Human-readable name for this step (logging, debugging, observability).
    fn name(&self) -> &str;
}
