//! [`SnapshotStoreFactory`] — constructor contract for snapshot store implementations.

use std::fmt::Display;

use crate::api::snapshot::traits::Snapshot;
use crate::api::snapshot::types::InMemorySnapshotStore;

/// Factory trait for the standard [`SnapshotStore`](crate::SnapshotStore) implementations.
pub trait SnapshotStoreFactory {
    /// Construct an [`InMemorySnapshotStore`] for development and testing.
    fn in_memory<S>() -> InMemorySnapshotStore<S>
    where
        S: Snapshot + Clone,
        S::AggregateId: Display,
    {
        InMemorySnapshotStore::new()
    }
}
