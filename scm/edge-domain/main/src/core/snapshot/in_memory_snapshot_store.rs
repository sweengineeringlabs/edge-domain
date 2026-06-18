//! In-memory snapshot store — keeps the latest snapshot per aggregate.
//!
//! A reference [`SnapshotStore`] for development and testing.  State lives in
//! process memory and is lost when the process stops.

use std::collections::HashMap;
use std::fmt::Display;

use futures::future::BoxFuture;
use parking_lot::RwLock;

use crate::api::SnapshotError;
use crate::api::Snapshot;
use crate::api::SnapshotStore;

// impl Snapshot for NoopSnapshot (see noop_snapshot.rs)

pub(crate) struct InMemorySnapshotStore<S: Snapshot> {
    snapshots: RwLock<HashMap<S::AggregateId, S>>,
}

impl<S: Snapshot> InMemorySnapshotStore<S> {
    pub(crate) fn new() -> Self {
        Self {
            snapshots: RwLock::new(HashMap::new()),
        }
    }
}

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
    struct InMemorySnapshotStoreTestSnap {
        aggregate_id: String,
        version: u64,
    }

    impl Snapshot for InMemorySnapshotStoreTestSnap {
        type AggregateId = String;
        fn aggregate_id(&self) -> &Self::AggregateId {
            &self.aggregate_id
        }
        fn version(&self) -> u64 {
            self.version
        }
    }

    fn snap(id: &str, version: u64) -> InMemorySnapshotStoreTestSnap {
        InMemorySnapshotStoreTestSnap {
            aggregate_id: id.to_string(),
            version,
        }
    }

    #[test]
    fn test_new_creates_empty_store() {
        let store = InMemorySnapshotStore::<InMemorySnapshotStoreTestSnap>::new();
        assert!(store.snapshots.read().is_empty());
    }

    #[tokio::test]
    async fn test_save_then_load_returns_snapshot() {
        let store = InMemorySnapshotStore::new();
        store.save(snap("a1", 5)).await.unwrap();
        let loaded = store.load(&"a1".to_string()).await.unwrap();
        assert_eq!(loaded.map(|s| s.version), Some(5));
    }

    #[tokio::test]
    async fn test_load_missing_returns_none() {
        let store = InMemorySnapshotStore::<InMemorySnapshotStoreTestSnap>::new();
        assert!(store.load(&"ghost".to_string()).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_save_version_zero_returns_invalid_version() {
        let store = InMemorySnapshotStore::new();
        let err = store.save(snap("a1", 0)).await.unwrap_err();
        assert_eq!(
            err,
            SnapshotError::InvalidVersion {
                aggregate_id: "a1".to_string(),
                version: 0
            }
        );
    }

    #[tokio::test]
    async fn test_save_overwrites_earlier_snapshot() {
        let store = InMemorySnapshotStore::new();
        store.save(snap("a1", 3)).await.unwrap();
        store.save(snap("a1", 8)).await.unwrap();
        let loaded = store.load(&"a1".to_string()).await.unwrap();
        assert_eq!(loaded.map(|s| s.version), Some(8));
    }
}
