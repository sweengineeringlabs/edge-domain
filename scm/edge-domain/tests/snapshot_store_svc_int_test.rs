//! Integration tests for `SnapshotStore` and the `new_in_memory_snapshot_store`
//! SAF factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Domain, Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotError,
    SnapshotLoadRequest, SnapshotLoadResponse, SnapshotSaveRequest, SnapshotStore,
    SnapshotVersionRequest, SnapshotVersionResponse,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
struct AccountSnapshot {
    account_id: String,
    version: u64,
    balance: i64,
}

impl Snapshot for AccountSnapshot {
    type AggregateId = String;
    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.account_id,
        })
    }
    fn version(
        &self,
        _req: SnapshotVersionRequest,
    ) -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse {
            version: self.version,
        })
    }
}

fn snapshot(id: &str, version: u64, balance: i64) -> AccountSnapshot {
    AccountSnapshot {
        account_id: id.to_string(),
        version,
        balance,
    }
}

fn store() -> std::sync::Arc<dyn SnapshotStore<AggregateId = String, Snap = AccountSnapshot>> {
    Domain.new_in_memory_snapshot_store::<AccountSnapshot>()
}

/// A consumer store whose backing medium is always unreachable — used to prove
/// callers can surface a `SnapshotError` from the load path.
struct UnavailableSnapshotStore;

impl SnapshotStore for UnavailableSnapshotStore {
    type AggregateId = String;
    type Snap = AccountSnapshot;

    fn save(
        &self,
        _req: SnapshotSaveRequest<Self::Snap>,
    ) -> BoxFuture<'_, Result<(), SnapshotError>> {
        Box::pin(async move { Err(SnapshotError::Unavailable("disk offline".to_string())) })
    }

    fn load<'a>(
        &'a self,
        _req: SnapshotLoadRequest<'a, Self::AggregateId>,
    ) -> BoxFuture<'a, Result<SnapshotLoadResponse<Self::Snap>, SnapshotError>> {
        Box::pin(async move { Err(SnapshotError::Unavailable("disk offline".to_string())) })
    }
}

// ---- new_in_memory_snapshot_store (Rule 221) ----

/// @covers: new_in_memory_snapshot_store
#[test]
fn test_new_in_memory_snapshot_store_round_trips_snapshot_happy() {
    let store = store();
    block_on(store.save(SnapshotSaveRequest {
        snapshot: snapshot("acct-1", 5, 100),
    }))
    .unwrap();
    let id = "acct-1".to_string();
    let loaded = block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot;
    assert_eq!(loaded.map(|s| s.balance), Some(100));
}

/// @covers: new_in_memory_snapshot_store
#[test]
fn test_new_in_memory_snapshot_store_rejects_version_zero_error() {
    let store = store();
    let err = block_on(store.save(SnapshotSaveRequest {
        snapshot: snapshot("acct-1", 0, 0),
    }))
    .unwrap_err();
    assert_eq!(
        err,
        SnapshotError::InvalidVersion {
            aggregate_id: "acct-1".to_string(),
            version: 0
        }
    );
}

/// @covers: new_in_memory_snapshot_store
#[test]
fn test_new_in_memory_snapshot_store_load_missing_returns_none_edge() {
    let store = store();
    let id = "never-saved".to_string();
    assert!(block_on(store.load(SnapshotLoadRequest { id: &id }))
        .unwrap()
        .snapshot
        .is_none());
}

// ---- SnapshotStore::save (Rule 222) ----

/// @covers: SnapshotStore::save
#[tokio::test]
async fn test_save_valid_snapshot_returns_ok_happy() {
    let store = store();
    assert!(store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 3, 50)
        })
        .await
        .is_ok());
}

/// @covers: SnapshotStore::save
#[tokio::test]
async fn test_save_to_unavailable_store_returns_error() {
    let store = UnavailableSnapshotStore;
    let err = store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 3, 50),
        })
        .await
        .unwrap_err();
    assert_eq!(err, SnapshotError::Unavailable("disk offline".to_string()));
}

/// @covers: SnapshotStore::save
#[tokio::test]
async fn test_save_twice_keeps_latest_snapshot_edge() {
    let store = store();
    store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 3, 50),
        })
        .await
        .unwrap();
    store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 9, 75),
        })
        .await
        .unwrap();
    let id = "acct-1".to_string();
    let loaded = store
        .load(SnapshotLoadRequest { id: &id })
        .await
        .unwrap()
        .snapshot
        .unwrap();
    assert_eq!((loaded.version, loaded.balance), (9, 75));
}

// ---- SnapshotStore::load (Rule 222) ----

/// @covers: SnapshotStore::load
#[tokio::test]
async fn test_load_existing_snapshot_returns_some_happy() {
    let store = store();
    store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 7, 200),
        })
        .await
        .unwrap();
    let id = "acct-1".to_string();
    let loaded = store
        .load(SnapshotLoadRequest { id: &id })
        .await
        .unwrap()
        .snapshot;
    assert_eq!(loaded.map(|s| s.version), Some(7));
}

/// @covers: SnapshotStore::load
#[tokio::test]
async fn test_load_from_unavailable_store_propagates_error() {
    let store = UnavailableSnapshotStore;
    let id = "acct-1".to_string();
    let err = store
        .load(SnapshotLoadRequest { id: &id })
        .await
        .unwrap_err();
    assert_eq!(err, SnapshotError::Unavailable("disk offline".to_string()));
}

/// @covers: SnapshotStore::load
#[tokio::test]
async fn test_load_unknown_aggregate_returns_none_edge() {
    let store = store();
    store
        .save(SnapshotSaveRequest {
            snapshot: snapshot("acct-1", 1, 10),
        })
        .await
        .unwrap();
    let id = "acct-2".to_string();
    assert!(store
        .load(SnapshotLoadRequest { id: &id })
        .await
        .unwrap()
        .snapshot
        .is_none());
}
