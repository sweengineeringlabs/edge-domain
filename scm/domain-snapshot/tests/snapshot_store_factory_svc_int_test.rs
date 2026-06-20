//! SAF facade tests — `SnapshotStoreBootstrap` constructors.
// @allow: no_mocks_in_integration — InMemorySnapshotStore is the production-shipped reference impl, not a test double

use edge_domain_snapshot::{InMemorySnapshotStore, Snapshot, SnapshotStoreBootstrap};

#[derive(Clone)]
struct OrderSnapshot { id: String, version: u64 }
impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(&self) -> &Self::AggregateId { &self.id }
    fn version(&self) -> u64 { self.version }
}

struct SnapshotStores;
impl SnapshotStoreBootstrap for SnapshotStores {}

/// @covers: SnapshotStoreBootstrap::in_memory — returns usable store
#[test]
fn test_in_memory_returns_empty_store_happy() {
    let store: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    assert!(store.snapshots.read().is_empty());
}

/// @covers: SnapshotStoreBootstrap::in_memory — returns different instances
#[test]
fn test_in_memory_independent_instances_are_isolated_error() {
    let a: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    let b: InMemorySnapshotStore<OrderSnapshot> = SnapshotStores::in_memory();
    assert!(a.snapshots.read().is_empty());
    assert!(b.snapshots.read().is_empty());
}

/// @covers: SnapshotStoreBootstrap::in_memory — Default trait works
#[test]
fn test_in_memory_default_creates_empty_store_edge() {
    let store: InMemorySnapshotStore<OrderSnapshot> = Default::default();
    assert!(store.snapshots.read().is_empty());
}
