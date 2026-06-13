//! `Snapshot` theme — point-in-time aggregate state for replay optimisation.

pub mod errors;
pub mod noop_snapshot;
pub mod std_snapshot_store_factory;
pub mod traits;
pub mod types;

pub use errors::SnapshotError;
pub use traits::{Snapshot, SnapshotStore, SnapshotStoreFactory};
pub use types::{InMemorySnapshotStore, NoopSnapshot, StdSnapshotStoreFactory};
