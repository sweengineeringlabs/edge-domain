//! [`ParallelConfig`] — configuration for parallel step fan-out execution.

use std::time::Duration;

/// Configuration for a parallel step fan-out's execution behavior.
#[derive(Clone, Debug, Default)]
pub struct ParallelConfig {
    /// Per-branch execution timeout. `None` disables per-branch timeout.
    pub timeout_per_branch: Option<Duration>,

    /// Cancel remaining branches and return immediately on the first branch failure
    /// when `true`. Let every branch run to completion and collect all failures when
    /// `false` (default).
    pub fail_fast: bool,

    /// Emit lifecycle events when `true`; requires an event bus attached via
    /// [`ParallelStepBuilder::with_event_bus`](crate::api::ParallelStepBuilder).
    pub emit_lifecycle_events: bool,
}
