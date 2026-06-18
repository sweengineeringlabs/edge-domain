//! SAF — snapshot service facade.
#[cfg(not(feature = "snapshot"))]
pub use crate::api::Snapshot;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::SnapshotError;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::SnapshotStore;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SNAPSHOT_SVC: () = ();
