//! Integration tests for `InMemorySnapshotStore` — covers the type file directly.
// @allow: no_mocks_in_integration — InMemorySnapshotStore is the production-shipped reference impl, not a test double

use edge_domain_snapshot::{InMemorySnapshotStore, Snapshot, SnapshotStore};
use futures::executor::block_on;

#[derive(Clone)]
struct OrderSnapshot { id: String, version: u64 }
impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(&self) -> &Self::AggregateId { &self.id }
    fn version(&self) -> u64 { self.version }
}

fn order_snapshot(id: &str, v: u64) -> OrderSnapshot { OrderSnapshot { id: id.to_string(), version: v } }

/// @covers: InMemorySnapshotStore::new — creates empty store
#[test]
fn test_new_creates_empty_store_happy() {
    let store = InMemorySnapshotStore::<OrderSnapshot>::new();
    assert!(store.snapshots.read().is_empty());
}

/// @covers: InMemorySnapshotStore — load missing returns None
#[test]
fn test_load_missing_aggregate_returns_none_error() {
    let store = InMemorySnapshotStore::<OrderSnapshot>::new();
    let result = block_on(store.load(&"ghost".to_string())).unwrap();
    assert!(result.is_none());
}

/// @covers: InMemorySnapshotStore — isolates different aggregates
#[test]
fn test_load_different_aggregate_id_returns_none_edge() {
    let store = InMemorySnapshotStore::<OrderSnapshot>::new();
    block_on(store.save(order_snapshot("a", 1))).unwrap();
    let result = block_on(store.load(&"b".to_string())).unwrap();
    assert!(result.is_none());
}
