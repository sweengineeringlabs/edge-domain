//! [`ParallelStepError<E>`] — aggregate error returned when one or more branches of a
//! parallel step fan-out did not succeed.

use crate::api::ParallelBranchFailure;

/// Engine-owned aggregate error for parallel step execution.
///
/// Non-empty by construction — a parallel step only ever returns `Err` when at least one
/// branch failed or timed out. Deliberately distinct from the bare `E` used by sequential
/// [`Step`](crate::Step) implementors: collapsing multiple branch failures into a single `E`
/// slot would silently discard all but one of them. Consumers that need to nest a parallel
/// step into an outer pipeline with a different error type write a small adapter mapping
/// `ParallelStepError<E>` to that outer `E` — the same pattern already used for
/// [`Pipeline`](crate::Pipeline)-as-[`Step`](crate::Step) nesting.
#[derive(Debug)]
pub struct ParallelStepError<E> {
    /// Every branch that failed or timed out, in the order they were detected.
    pub failures: Vec<ParallelBranchFailure<E>>,
}
