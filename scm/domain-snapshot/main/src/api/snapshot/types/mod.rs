//! Snapshot theme — neutral implementation types.

pub mod in_memory_snapshot_store;
pub mod noop_snapshot;
pub mod snapshot_aggregate_id_request;
pub mod snapshot_aggregate_id_response;
pub mod snapshot_load_request;
pub mod snapshot_load_response;
pub mod snapshot_save_request;
pub mod snapshot_version_request;
pub mod snapshot_version_response;

pub use in_memory_snapshot_store::InMemorySnapshotStore;
pub use noop_snapshot::NoopSnapshot;
pub use snapshot_aggregate_id_request::SnapshotAggregateIdRequest;
pub use snapshot_aggregate_id_response::SnapshotAggregateIdResponse;
pub use snapshot_load_request::SnapshotLoadRequest;
pub use snapshot_load_response::SnapshotLoadResponse;
pub use snapshot_save_request::SnapshotSaveRequest;
pub use snapshot_version_request::SnapshotVersionRequest;
pub use snapshot_version_response::SnapshotVersionResponse;
