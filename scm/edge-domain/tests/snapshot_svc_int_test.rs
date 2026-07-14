//! SAF facade integration tests — the `Snapshot` trait is exported from the
//! crate root and implementable by downstream consumers.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Snapshot, SnapshotAggregateIdRequest, SnapshotAggregateIdResponse, SnapshotError,
    SnapshotVersionRequest, SnapshotVersionResponse,
};

/// A consumer-defined aggregate snapshot.
#[derive(Clone)]
struct OrderSnapshot {
    order_id: String,
    version: u64,
    line_items: u32,
}

impl Snapshot for OrderSnapshot {
    type AggregateId = String;
    fn aggregate_id(
        &self,
        _req: SnapshotAggregateIdRequest,
    ) -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse {
            aggregate_id: &self.order_id,
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

/// @covers: Snapshot::aggregate_id
#[test]
fn test_aggregate_id_returns_owning_aggregate_happy() {
    let snap = OrderSnapshot {
        order_id: "order-42".to_string(),
        version: 10,
        line_items: 3,
    };
    assert_eq!(
        snap.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "order-42"
    );
}

/// @covers: Snapshot::aggregate_id
#[test]
fn test_aggregate_id_distinguishes_snapshots_of_different_aggregates_edge() {
    let a = OrderSnapshot {
        order_id: "order-1".to_string(),
        version: 1,
        line_items: 0,
    };
    let b = OrderSnapshot {
        order_id: "order-2".to_string(),
        version: 1,
        line_items: 0,
    };
    assert_ne!(
        a.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        b.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id
    );
}

/// @covers: Snapshot::aggregate_id
#[test]
fn test_aggregate_id_stable_across_clone_error() {
    // A cloned snapshot that drifts from its source would corrupt replay
    // resumption — guard against it.
    let snap = OrderSnapshot {
        order_id: "order-7".to_string(),
        version: 4,
        line_items: 9,
    };
    let cloned = snap.clone();
    assert_eq!(
        snap.aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        cloned
            .aggregate_id(SnapshotAggregateIdRequest)
            .unwrap()
            .aggregate_id
    );
    assert_eq!(cloned.line_items, 9);
}

/// @covers: Snapshot::version
#[test]
fn test_version_returns_captured_stream_version_happy() {
    let snap = OrderSnapshot {
        order_id: "order-1".to_string(),
        version: 27,
        line_items: 5,
    };
    assert_eq!(snap.version(SnapshotVersionRequest).unwrap().version, 27);
}

/// @covers: Snapshot::version
#[test]
fn test_version_one_is_the_minimum_meaningful_value_edge() {
    let snap = OrderSnapshot {
        order_id: "order-1".to_string(),
        version: 1,
        line_items: 1,
    };
    assert_eq!(snap.version(SnapshotVersionRequest).unwrap().version, 1);
}

/// @covers: Snapshot::version
#[test]
fn test_version_zero_signals_nothing_to_snapshot_error() {
    // version 0 is the sentinel the store rejects — see SnapshotError.
    let snap = OrderSnapshot {
        order_id: "order-1".to_string(),
        version: 0,
        line_items: 0,
    };
    assert_eq!(snap.version(SnapshotVersionRequest).unwrap().version, 0);
}
