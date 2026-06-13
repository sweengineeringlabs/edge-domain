//! Snapshot theme — neutral implementation types.

pub mod in_memory_snapshot_store;
pub mod noop_snapshot;
pub mod std_snapshot_store_factory;

pub use in_memory_snapshot_store::InMemorySnapshotStore;
pub use noop_snapshot::NoopSnapshot;
pub use std_snapshot_store_factory::StdSnapshotStoreFactory;
