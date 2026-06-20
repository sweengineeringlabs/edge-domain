use edge_domain_snapshot::{Snapshot, SnapshotStoreBootstrap, StdSnapshotStoreFactory};

#[test]
fn test_noop_snapshot_returns_default_version_happy() {
    let s = StdSnapshotStoreFactory::noop_snapshot();
    assert_eq!(s.version(), 0);
}

#[test]
fn test_noop_snapshot_empty_id_signals_no_aggregate_error() {
    let s = StdSnapshotStoreFactory::noop_snapshot();
    assert!(s.aggregate_id().is_empty());
}

#[test]
fn test_noop_snapshot_clone_is_independent_edge() {
    let s = StdSnapshotStoreFactory::noop_snapshot();
    let cloned = s.clone();
    assert_eq!(s.version(), cloned.version());
}

#[test]
fn test_std_factory_returns_instance_happy() {
    let _ = StdSnapshotStoreFactory::std_factory();
}

#[test]
fn test_std_factory_is_copy_type_error() {
    let f = StdSnapshotStoreFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — still usable after move
}

#[test]
fn test_std_factory_in_memory_creates_empty_store_edge() {
    use edge_domain_snapshot::{NoopSnapshot, SnapshotStore};
    let store = StdSnapshotStoreFactory::in_memory::<NoopSnapshot>();
    let result = futures::executor::block_on(store.load(&String::new())).unwrap();
    assert!(result.is_none());
}
