//! `Snapshot` theme — point-in-time aggregate state for replay optimisation.

pub mod errors;
pub mod noop_snapshot;
pub mod std_snapshot_store_factory;
pub mod traits;
pub mod types;

pub use errors::SnapshotError;
pub use noop_snapshot::NoopSnapshot;
pub use std_snapshot_store_factory::StdSnapshotStoreFactory;
pub use traits::{Snapshot, SnapshotStore, SnapshotStoreBootstrap};
pub use types::InMemorySnapshotStore;
