//! `SnapshotStore` — persists and retrieves [`Snapshot`]s.

use std::hash::Hash;

use futures::future::BoxFuture;

use crate::api::snapshot::errors::SnapshotError;
use crate::api::snapshot::traits::Snapshot;

/// Stores and retrieves [`Snapshot`]s keyed by aggregate id.
///
/// Uses [`BoxFuture`] for async methods, consistent with
/// [`EventStore`](crate::EventStore) and [`Repository`](crate::Repository).
/// The associated [`AggregateId`](SnapshotStore::AggregateId) is constrained to
/// equal the snapshot's own id type, preventing mismatched id types at the call
/// site.
pub trait SnapshotStore: Send + Sync {
    /// The aggregate identity type snapshots are keyed by.
    type AggregateId: Eq + Hash + Clone + Send + Sync;

    /// The snapshot type stored, keyed by the same [`AggregateId`].
    ///
    /// [`AggregateId`]: SnapshotStore::AggregateId
    type Snap: Snapshot<AggregateId = Self::AggregateId>;

    /// Persist a snapshot, replacing any earlier snapshot for the same aggregate.
    fn save(&self, snapshot: Self::Snap) -> BoxFuture<'_, Result<(), SnapshotError>>;

    /// Load the latest snapshot for `id`, or `None` if none has been saved.
    fn load(
        &self,
        id: &Self::AggregateId,
    ) -> BoxFuture<'_, Result<Option<Self::Snap>, SnapshotError>>;
}
