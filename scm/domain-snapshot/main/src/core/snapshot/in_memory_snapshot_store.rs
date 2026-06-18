//! `SnapshotStore` impl for [`InMemorySnapshotStore`].

use std::fmt::Display;

use futures::future::BoxFuture;

use crate::api::SnapshotError;
use crate::api::{Snapshot, SnapshotStore};
use crate::api::InMemorySnapshotStore;

impl<S> SnapshotStore for InMemorySnapshotStore<S>
where
    S: Snapshot + Clone,
    S::AggregateId: Display,
{
    type AggregateId = S::AggregateId;
    type Snap = S;

    fn save(&self, snapshot: Self::Snap) -> BoxFuture<'_, Result<(), SnapshotError>> {
        let version = snapshot.version();
        if version == 0 {
            let aggregate_id = snapshot.aggregate_id().to_string();
            return Box::pin(async move {
                Err(SnapshotError::InvalidVersion {
                    aggregate_id,
                    version,
                })
            });
        }
        let key = snapshot.aggregate_id().clone();
        self.snapshots.write().insert(key, snapshot);
        Box::pin(async move { Ok(()) })
    }

    fn load(
        &self,
        id: &Self::AggregateId,
    ) -> BoxFuture<'_, Result<Option<Self::Snap>, SnapshotError>> {
        let found = self.snapshots.read().get(id).cloned();
        Box::pin(async move { Ok(found) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct InMemorySnapshotStoreOrderFixture {
        aggregate_id: String,
        version: u64,
    }

    impl Snapshot for InMemorySnapshotStoreOrderFixture {
        type AggregateId = String;
        fn aggregate_id(&self) -> &Self::AggregateId {
            &self.aggregate_id
        }
        fn version(&self) -> u64 {
            self.version
        }
    }

    fn fixture(id: &str, v: u64) -> InMemorySnapshotStoreOrderFixture {
        InMemorySnapshotStoreOrderFixture { aggregate_id: id.to_string(), version: v }
    }

    #[test]
    fn test_save_new_snapshot_inserts_entry_happy() {
        let store = InMemorySnapshotStore::new();
        futures::executor::block_on(store.save(fixture("agg-1", 3))).unwrap();
        assert!(store.snapshots.read().contains_key("agg-1"));
    }

    #[test]
    fn test_save_version_zero_returns_invalid_version_error() {
        let store = InMemorySnapshotStore::<InMemorySnapshotStoreOrderFixture>::new();
        let err = futures::executor::block_on(store.save(fixture("agg-1", 0))).unwrap_err();
        assert!(matches!(err, SnapshotError::InvalidVersion { .. }));
    }

    #[test]
    fn test_load_absent_aggregate_returns_none_edge() {
        let store = InMemorySnapshotStore::<InMemorySnapshotStoreOrderFixture>::new();
        let result = futures::executor::block_on(store.load(&"absent".to_string())).unwrap();
        assert!(result.is_none());
    }
}
