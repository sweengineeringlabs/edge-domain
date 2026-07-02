//! [`ParallelExecutor`] — runs a set of branch steps concurrently.

use crate::api::{
    ParallelConfig, ParallelStepBuilder, ParallelStepError, PipelineError, Step, StepCountRequest,
    StepCountResponse,
};

/// A [`Step`] that fans out to N branch steps and runs them concurrently.
///
/// `Self::ExecutionError` (inherited from the [`Step`] supertrait) is always
/// `ParallelStepError<Self::BranchError>` — the engine-owned aggregate that wraps every
/// branch's outcome. See RFC-002 (`docs/3-architecture/rfc/RFC-002-parallel-step-execution.md`)
/// for the full design: why `Self::Ctx` must be `Clone`, why branch failures are collected
/// into an aggregate rather than silently reduced to one, and why composing a
/// `ParallelExecutor` into an outer `Pipeline` with a different error type requires a small
/// consumer-written adapter.
pub trait ParallelExecutor: Step<ExecutionError = ParallelStepError<Self::BranchError>> {
    /// The consumer's per-branch domain error type.
    type BranchError: Send + 'static;

    /// Return the number of branch steps in this fan-out.
    fn branch_count(
        &self,
        req: StepCountRequest,
    ) -> Result<StepCountResponse, PipelineError<Self::BranchError>>;

    /// Create a new fluent builder for assembling a parallel step fan-out.
    fn new_builder() -> ParallelStepBuilder<Self::Ctx, Self::BranchError>
    where
        Self: Sized,
    {
        ParallelStepBuilder {
            steps: Vec::new(),
            config: ParallelConfig::default(),
            event_bus: None,
        }
    }
}
