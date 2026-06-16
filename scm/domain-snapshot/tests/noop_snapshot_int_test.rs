use edge_domain_snapshot::{NoopSnapshot, Snapshot};

#[test]
fn test_noop_snapshot_aggregate_id_returns_empty_string_happy() {
    let s = NoopSnapshot::default();
    assert_eq!(s.aggregate_id(), "");
}

#[test]
fn test_noop_snapshot_version_returns_zero_happy() {
    let s = NoopSnapshot::default();
    assert_eq!(s.version(), 0);
}

#[test]
fn test_noop_snapshot_clone_produces_equal_value_edge() {
    let s = NoopSnapshot::default();
    let cloned = s.clone();
    assert_eq!(s.aggregate_id(), cloned.aggregate_id());
    assert_eq!(s.version(), cloned.version());
}
