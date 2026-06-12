//! SAF facade tests — `Snapshot` trait.

use edge_domain_snapshot::Snapshot;

struct OrderSnapshot {
    id: String,
    version: u64,
}

impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(&self) -> &Self::AggregateId {
        &self.id
    }
    fn version(&self) -> u64 {
        self.version
    }
}

/// @covers: Snapshot::aggregate_id — returns the configured id
#[test]
fn test_aggregate_id_configured_value_returned_happy() {
    let snap = OrderSnapshot { id: "order-1".into(), version: 5 };
    assert_eq!(snap.aggregate_id(), "order-1");
}

/// @covers: Snapshot::aggregate_id — empty id is preserved as-is
#[test]
fn test_aggregate_id_empty_string_preserved_error() {
    let snap = OrderSnapshot { id: String::new(), version: 1 };
    assert_eq!(snap.aggregate_id(), "");
}

/// @covers: Snapshot — aggregate_id and version are consistent
#[test]
fn test_aggregate_id_and_version_are_consistent_edge() {
    let snap = OrderSnapshot { id: "order-99".into(), version: 42 };
    assert_eq!(snap.aggregate_id(), "order-99");
    assert_eq!(snap.version(), 42);
}

/// @covers: Snapshot::version — returns the configured version
#[test]
fn test_version_configured_value_returned_happy() {
    let snap = OrderSnapshot { id: "x".into(), version: 7 };
    assert_eq!(snap.version(), 7);
}

/// @covers: Snapshot::version — returns zero for version-zero snap
#[test]
fn test_version_zero_returns_zero_error() {
    let snap = OrderSnapshot { id: "x".into(), version: 0 };
    assert_eq!(snap.version(), 0);
}

/// @covers: Snapshot::version — returns max u64 without overflow
#[test]
fn test_version_max_u64_preserved_edge() {
    let snap = OrderSnapshot { id: "x".into(), version: u64::MAX };
    assert_eq!(snap.version(), u64::MAX);
}
