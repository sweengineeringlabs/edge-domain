//! `StdSnapshotStoreFactory` — the canonical [`SnapshotStoreFactory`](crate::SnapshotStoreFactory) marker.

/// Canonical marker that implements the standard [`SnapshotStoreFactory`](crate::SnapshotStoreFactory) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdSnapshotStoreFactory;
