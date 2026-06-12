//! `Snapshot` theme — point-in-time aggregate state for replay optimisation.
//!
//! Owns the [`Snapshot`] and [`SnapshotStore`] contracts and the
//! [`SnapshotError`] type.  The in-memory reference store is obtained from the
//! [`Domain::new_in_memory_snapshot_store`](crate::Domain::new_in_memory_snapshot_store)
//! factory, which returns an `Arc<dyn SnapshotStore>` — there is no public
//! marker type (see edge-domain#9).  Concrete snapshots are defined by
//! consumers for their own aggregates and implement [`Snapshot`] there.

pub mod errors;
pub mod traits;

pub use errors::SnapshotError;
pub use traits::{Snapshot, SnapshotStore};
