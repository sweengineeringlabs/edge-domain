//! `Snapshot` — a point-in-time capture of aggregate state.

use std::hash::Hash;

/// A point-in-time capture of aggregate state used to short-circuit full event
/// replay.
///
/// When an aggregate has a long event history, replaying every event on each
/// load is expensive.  A snapshot records the aggregate's state at a specific
/// version so that only events *after* that version need to be replayed.
pub trait Snapshot: Send + Sync {
    /// The aggregate identity type this snapshot is keyed by.
    type AggregateId: Eq + Hash + Clone + Send + Sync;

    /// The aggregate id this snapshot belongs to.
    fn aggregate_id(&self) -> &Self::AggregateId;

    /// The event stream version at the time this snapshot was taken.
    ///
    /// Replaying resumes from `version + 1`.  A snapshot is only meaningful at
    /// version `>= 1` — there is nothing to capture before the first event.
    fn version(&self) -> u64;
}
