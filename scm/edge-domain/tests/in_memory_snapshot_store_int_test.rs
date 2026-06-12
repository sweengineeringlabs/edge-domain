//! Integration tests for the in-memory snapshot store implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, Snapshot, SnapshotError, SnapshotStore};

#[derive(Clone)]
struct CounterSnapshot {
    id: String,
    version: u64,
    count: u64,
}

impl Snapshot for CounterSnapshot {
    type AggregateId = String;
    fn aggregate_id(&self) -> &Self::AggregateId {
        &self.id
    }
    fn version(&self) -> u64 {
        self.version
    }
}

fn store() -> std::sync::Arc<dyn SnapshotStore<AggregateId = String, Snap = CounterSnapshot>> {
    Domain::new_in_memory_snapshot_store::<CounterSnapshot>()
}

/// @covers: new_in_memory_snapshot_store
#[tokio::test]
async fn test_in_memory_snapshot_store_persists_and_retrieves_latest() {
    let store = store();
    store
        .save(CounterSnapshot {
            id: "c1".to_string(),
            version: 12,
            count: 99,
        })
        .await
        .unwrap();
    let loaded = store.load(&"c1".to_string()).await.unwrap().unwrap();
    assert_eq!((loaded.version, loaded.count), (12, 99));
}

/// @covers: new_in_memory_snapshot_store
#[tokio::test]
async fn test_in_memory_snapshot_store_returns_none_for_unknown_aggregate() {
    let store = store();
    assert!(store.load(&"missing".to_string()).await.unwrap().is_none());
}

/// @covers: new_in_memory_snapshot_store
#[tokio::test]
async fn test_in_memory_snapshot_store_rejects_zero_version_snapshot() {
    let store = store();
    let err = store
        .save(CounterSnapshot {
            id: "c1".to_string(),
            version: 0,
            count: 0,
        })
        .await
        .unwrap_err();
    assert_eq!(
        err,
        SnapshotError::InvalidVersion {
            aggregate_id: "c1".to_string(),
            version: 0
        }
    );
}
