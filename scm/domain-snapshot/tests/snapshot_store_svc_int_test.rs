//! SAF facade tests — `SnapshotStore` trait via `InMemorySnapshotStore`.
// @allow: no_mocks_in_integration — InMemorySnapshotStore is the production-shipped reference impl, not a test double

use edge_domain_snapshot::{InMemorySnapshotStore, Snapshot, SnapshotError, SnapshotStore, SnapshotStoreFactory};
use futures::executor::block_on;

#[derive(Clone)]
struct OrderSnapshot {
    id: String,
    version: u64,
}

impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(&self) -> &Self::AggregateId { &self.id }
    fn version(&self) -> u64 { self.version }
}

struct SnapshotStores;
impl SnapshotStoreFactory for SnapshotStores {}

fn order_snapshot(id: &str, v: u64) -> OrderSnapshot {
    OrderSnapshot { id: id.to_string(), version: v }
}

/// @covers: SnapshotStore::save + load — round-trip
#[test]
fn test_save_then_load_returns_saved_snapshot_happy() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    block_on(store.save(order_snapshot("a1", 5))).unwrap();
    let loaded = block_on(store.load(&"a1".to_string())).unwrap();
    assert_eq!(loaded.map(|s| s.version), Some(5));
}

/// @covers: SnapshotStore::save — version zero is rejected
#[test]
fn test_save_version_zero_returns_invalid_version_error() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    let err = block_on(store.save(order_snapshot("a1", 0))).unwrap_err();
    assert_eq!(err, SnapshotError::InvalidVersion { aggregate_id: "a1".into(), version: 0 });
}

/// @covers: SnapshotStore — save overwrites earlier snapshot
#[test]
fn test_save_overwrites_earlier_snapshot_edge() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    block_on(store.save(order_snapshot("a1", 3))).unwrap();
    block_on(store.save(order_snapshot("a1", 8))).unwrap();
    let loaded = block_on(store.load(&"a1".to_string())).unwrap();
    assert_eq!(loaded.map(|s| s.version), Some(8));
}

/// @covers: SnapshotStore::load — returns saved snapshot
#[test]
fn test_load_saved_snapshot_returns_some_happy() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    block_on(store.save(order_snapshot("b1", 10))).unwrap();
    let result = block_on(store.load(&"b1".to_string())).unwrap();
    assert!(result.is_some());
}

/// @covers: SnapshotStore::load — missing aggregate returns None
#[test]
fn test_load_missing_aggregate_returns_none_error() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    let result = block_on(store.load(&"missing".to_string())).unwrap();
    assert!(result.is_none());
}

/// @covers: SnapshotStore::load — isolates different aggregate ids
#[test]
fn test_load_different_aggregate_id_returns_none_edge() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    block_on(store.save(order_snapshot("c1", 2))).unwrap();
    let result = block_on(store.load(&"c2".to_string())).unwrap();
    assert!(result.is_none());
}
