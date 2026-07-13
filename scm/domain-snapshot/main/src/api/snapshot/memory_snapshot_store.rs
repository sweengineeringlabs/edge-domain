//! `MemorySnapshotStore` — SEA Rule 121 api/core mirror.
//!
//! `MemorySnapshotStore` is generic, so the structural auditor cannot match
//! the generic impl in `core/snapshot/memory_snapshot_store.rs` to an api
//! type by name. This path-level mirror provides the counterpart.
pub use crate::api::snapshot::types::MemorySnapshotStore;
