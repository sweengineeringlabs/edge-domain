//! [`ParallelBranchFailure<E>`] — one branch's failure outcome inside a parallel step fan-out.

use crate::api::StepError;

/// One branch's outcome within a parallel step fan-out, when it did not succeed.
///
/// Distinguishes a branch that returned `Err(E)` from a branch the engine timed out —
/// mirrors the split between [`PipelineError::StepFailed`](crate::PipelineError::StepFailed)
/// and [`PipelineError::StepTimeout`](crate::PipelineError::StepTimeout) at the sequential
/// pipeline level.
#[derive(Debug)]
pub enum ParallelBranchFailure<E> {
    /// The branch step returned `Err(cause)`.
    Failed(StepError<E>),
    /// The branch step exceeded its configured per-branch timeout.
    TimedOut {
        /// Name of the branch step that timed out.
        step_name: String,
    },
    /// The branch task panicked. Which branch cannot be attributed here — a panic means
    /// the task never produced a value to report its own name from; check application
    /// logs/tracing for the panic message (tokio's default panic hook reports it there).
    Panicked,
}
