//! [`SnapshotStoreFactory`] — constructor contract for snapshot store implementations.

use std::fmt::Display;

use crate::api::snapshot::traits::Snapshot;
use crate::api::snapshot::types::{InMemorySnapshotStore, NoopSnapshot, StdSnapshotStoreFactory};

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

    /// Return a no-op snapshot for testing and structural compliance.
    fn noop_snapshot() -> NoopSnapshot {
        NoopSnapshot::default()
    }

    /// Return the standard snapshot-store-factory instance.
    fn std_factory() -> StdSnapshotStoreFactory {
        StdSnapshotStoreFactory
    }
}
