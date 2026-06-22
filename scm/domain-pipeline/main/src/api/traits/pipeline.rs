//! [`Pipeline<Ctx>`] — orchestrates a sequence of steps.

use super::super::error::PipelineError;

/// Orchestrates a sequence of [`Step`] operations.
///
/// The pipeline executes steps in order, passing a mutable context through each step.
/// Each step enriches or validates the context. If any step fails, the pipeline
/// halts and returns the error.
///
/// # Invariant
///
/// Steps execute sequentially. The pipeline is not parallel.
#[async_trait::async_trait]
pub trait Pipeline<Ctx>: Send + Sync {
    /// Execute all steps in order.
    ///
    /// Steps are run sequentially. Context is mutated in-place by each step.
    /// If any step returns an error, execution stops and that error is returned.
    ///
    /// # Errors
    ///
    /// Returns the first [`PipelineError`] encountered. The context may be
    /// partially mutated from earlier steps.
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;

    /// Return the number of steps in this pipeline.
    fn step_count(&self) -> usize;

    /// Return true if the pipeline has no steps.
    fn is_empty(&self) -> bool {
        self.step_count() == 0
    }
}
