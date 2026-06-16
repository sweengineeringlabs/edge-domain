//! `InMemorySnapshotStore` — SEA Rule 121 api/core mirror.
//!
//! `InMemorySnapshotStore` is generic, so the structural auditor cannot match
//! the generic impl in `core/snapshot/in_memory_snapshot_store.rs` to an api
//! type by name. This path-level mirror provides the counterpart.
pub use crate::api::snapshot::types::InMemorySnapshotStore;
