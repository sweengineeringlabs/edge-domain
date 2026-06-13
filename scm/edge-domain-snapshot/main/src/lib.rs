//! # edge-domain-snapshot
//!
//! The `Snapshot` port contract — point-in-time aggregate state capture for event replay optimisation.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::InMemorySnapshotStore;
pub use saf::NoopSnapshot;
pub use saf::Snapshot;
pub use saf::SnapshotError;
pub use saf::SnapshotStore;
pub use saf::SnapshotStoreFactory;
pub use saf::StdSnapshotStoreFactory;
pub use saf::SNAPSHOT_SVC;
pub use saf::SNAPSHOT_STORE_SVC;
pub use saf::SNAPSHOT_STORE_FACTORY_SVC;
