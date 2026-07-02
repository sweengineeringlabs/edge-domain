//! Parallel step service — opaque construction surface for a concurrent step fan-out.

use std::sync::Arc;
use std::time::Duration;

use edge_domain_event::EventBus;

use crate::api::{ParallelConfig, ParallelExecutor, ParallelStepBuilder, ParallelStepError, Step};
use crate::core::traits::DefaultParallelStep;

/// Identifies the parallel-step `Service` implementation at runtime.
pub const PARALLEL_STEP_SVC: &str = "parallel_step";

/// Identifies the `ParallelStepSvc` factory implementation.
pub const PARALLEL_STEP_SVC_FACTORY: &str = "parallel_step_svc_factory";

/// Construction handle for a concurrent step fan-out.
///
/// Consumers declare a dependency on `Box<dyn Step<Ctx = Ctx, ExecutionError =
/// ParallelStepError<E>>>` (exclusive ownership) or the `Arc`-wrapped shared-ownership
/// equivalent. The concrete implementation (`DefaultParallelStep`) is never exposed.
///
/// Nesting a parallel step into an outer `Pipeline<Ctx, E>` with error type `E` (not
/// `ParallelStepError<E>`) requires a small consumer-written adapter — see RFC-002
/// (`docs/3-architecture/rfc/RFC-002-parallel-step-execution.md`) for why this is not
/// automatic, and `PipelineAsStep` in `tests/default_pipeline_int_test.rs` for the existing
/// precedent this follows.
pub struct ParallelStepSvc;

impl ParallelStepSvc {
    /// Build a parallel step fan-out with exclusive ownership.
    pub fn build<Ctx, E>(
        builder: ParallelStepBuilder<Ctx, E>,
    ) -> Box<dyn Step<Ctx = Ctx, ExecutionError = ParallelStepError<E>>>
    where
        Ctx: Clone + Send + 'static,
        E: Send + 'static,
    {
        let step = DefaultParallelStep::with_config(builder.steps, builder.config);
        let step = match builder.event_bus {
            Some(bus) => step.with_event_bus(bus),
            None => step,
        };
        Box::new(step)
    }

    /// Build a parallel step fan-out with shared ownership.
    pub fn build_shared<Ctx, E>(
        builder: ParallelStepBuilder<Ctx, E>,
    ) -> Arc<dyn Step<Ctx = Ctx, ExecutionError = ParallelStepError<E>>>
    where
        Ctx: Clone + Send + 'static,
        E: Send + 'static,
    {
        let step = DefaultParallelStep::with_config(builder.steps, builder.config);
        let step = match builder.event_bus {
            Some(bus) => step.with_event_bus(bus),
            None => step,
        };
        Arc::new(step)
    }

    /// Build a parallel step fan-out, typed as [`ParallelExecutor`] for callers that need
    /// `branch_count` (introspection) in addition to the `Step` contract. Use
    /// [`build`](Self::build) instead when the value only needs to compose into a
    /// `PipelineBuilder`/`ParallelStepBuilder`.
    pub fn build_executor<Ctx, E>(
        builder: ParallelStepBuilder<Ctx, E>,
    ) -> Box<dyn ParallelExecutor<Ctx = Ctx, BranchError = E, ExecutionError = ParallelStepError<E>>>
    where
        Ctx: Clone + Send + 'static,
        E: Send + 'static,
    {
        let step = DefaultParallelStep::with_config(builder.steps, builder.config);
        let step = match builder.event_bus {
            Some(bus) => step.with_event_bus(bus),
            None => step,
        };
        Box::new(step)
    }
}

impl<Ctx, E> ParallelStepBuilder<Ctx, E>
where
    Ctx: Send + 'static,
    E: Send + 'static,
{
    /// Create a new builder with default configuration and no branch steps.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            config: ParallelConfig::default(),
            event_bus: None,
        }
    }

    /// Append a branch step to run concurrently with the others.
    pub fn with<S: Step<Ctx = Ctx, ExecutionError = E> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Append a shared branch step (useful when the same step is reused elsewhere).
    pub fn with_shared(mut self, step: Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>) -> Self {
        self.steps.push(step);
        self
    }

    /// Apply a per-branch execution timeout; branches that exceed it are reported as
    /// [`ParallelBranchFailure::TimedOut`](crate::ParallelBranchFailure::TimedOut).
    pub fn timeout_per_branch(mut self, duration: Duration) -> Self {
        self.config.timeout_per_branch = Some(duration);
        self
    }

    /// Set whether the fan-out cancels remaining branches on the first failure.
    ///
    /// Defaults to `false` — every branch runs to completion and every failure is
    /// collected. Set to `true` to abort remaining branches and return immediately on the
    /// first failure or timeout.
    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.config.fail_fast = fail_fast;
        self
    }

    /// Enable or disable lifecycle event emission.
    ///
    /// Events are only published when [`with_event_bus`](Self::with_event_bus) is also
    /// called.
    pub fn emit_lifecycle_events(mut self, emit: bool) -> Self {
        self.config.emit_lifecycle_events = emit;
        self
    }

    /// Attach an event bus for lifecycle event emission.
    pub fn with_event_bus(mut self, bus: Arc<dyn EventBus>) -> Self {
        self.event_bus = Some(bus);
        self
    }
}
