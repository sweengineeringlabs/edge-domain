//! API-layer type for the in-memory snapshot store.

use std::collections::HashMap;
use std::hash::Hash;

use parking_lot::RwLock;

use crate::api::snapshot::traits::Snapshot;

/// An in-memory snapshot store — keeps the latest snapshot per aggregate.
///
/// A reference [`SnapshotStore`](crate::SnapshotStore) for development and
/// testing.  State lives in process memory and is lost when the process stops.
pub struct InMemorySnapshotStore<S: Snapshot> {
    /// The underlying snapshot map — exposed for test-time inspection.
    pub snapshots: RwLock<HashMap<S::AggregateId, S>>,
}

impl<S: Snapshot> InMemorySnapshotStore<S>
where
    S::AggregateId: Eq + Hash,
{
    /// Create a new empty in-memory snapshot store.
    pub fn new() -> Self {
        Self {
            snapshots: RwLock::new(HashMap::new()),
        }
    }
}

impl<S: Snapshot> Default for InMemorySnapshotStore<S>
where
    S::AggregateId: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
