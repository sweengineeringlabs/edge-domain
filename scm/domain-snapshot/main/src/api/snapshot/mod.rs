//! `Snapshot` theme — point-in-time aggregate state for replay optimisation.

pub mod errors;
pub mod noop_snapshot;
pub mod traits;
pub mod types;

pub use errors::SnapshotError;
pub use noop_snapshot::NoopSnapshot;
pub use traits::{Snapshot, SnapshotStore};
pub use types::{
    MemorySnapshotStore, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse,
    SnapshotLoadRequest, SnapshotLoadResponse, SnapshotSaveRequest, SnapshotVersionRequest,
    SnapshotVersionResponse,
};
