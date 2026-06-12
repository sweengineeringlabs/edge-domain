//! `Snapshot` — a point-in-time capture of aggregate state.

use std::hash::Hash;

/// A point-in-time capture of aggregate state used to short-circuit full event replay.
pub trait Snapshot: Send + Sync {
    /// The aggregate identity type this snapshot is keyed by.
    type AggregateId: Eq + Hash + Clone + Send + Sync;

    /// The aggregate id this snapshot belongs to.
    fn aggregate_id(&self) -> &Self::AggregateId;

    /// The event stream version at the time this snapshot was taken.
    fn version(&self) -> u64;
}
