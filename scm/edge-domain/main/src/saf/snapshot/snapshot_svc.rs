//! SAF — snapshot service facade.
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::Snapshot;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::SnapshotError;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::SnapshotStore;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SNAPSHOT_SVC: () = ();
