//! `StdSnapshotStoreFactory` — the canonical [`SnapshotStoreBootstrap`](crate::SnapshotStoreBootstrap) marker.

/// Canonical marker that implements the standard [`SnapshotStoreBootstrap`](crate::SnapshotStoreBootstrap) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdSnapshotStoreFactory;
