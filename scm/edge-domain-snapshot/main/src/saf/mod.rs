//! SAF — snapshot service facade.

mod snapshot;

pub use crate::api::snapshot::InMemorySnapshotStore;
pub use crate::api::snapshot::Snapshot;
pub use crate::api::snapshot::SnapshotError;
pub use crate::api::snapshot::SnapshotStore;
pub use crate::api::snapshot::SnapshotStoreFactory;
