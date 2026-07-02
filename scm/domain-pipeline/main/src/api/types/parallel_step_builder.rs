//! [`ParallelStepBuilder<Ctx, E>`] — fluent builder for assembling a parallel step fan-out.

use std::sync::Arc;

use edge_domain_event::EventBus;

use crate::api::{ParallelConfig, Step};

/// Fluent builder for assembling a parallel step fan-out.
///
/// `Ctx` must be `Clone` — each branch runs against its own independent clone of the
/// context; the engine performs no merge back into the caller's context. Consumers who
/// need branch output to be visible after the parallel block completes design that into
/// `Ctx` itself (fields needing cross-branch visibility behind `Arc<Mutex<T>>` /
/// `Arc<RwLock<T>>`).
///
/// Hand the completed builder to
/// [`ParallelStepSvc::build`](crate::ParallelStepSvc::build) to construct the concrete step.
pub struct ParallelStepBuilder<Ctx, E> {
    /// Branch steps to run concurrently.
    pub steps: Vec<Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>>,
    /// Accumulated parallel execution configuration.
    pub config: ParallelConfig,
    /// Optional event bus for lifecycle event emission.
    pub event_bus: Option<Arc<dyn EventBus>>,
}
