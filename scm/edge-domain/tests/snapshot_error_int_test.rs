//! Integration tests for `SnapshotError`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::SnapshotError;

/// @covers: SnapshotError
#[test]
fn test_snapshot_error_invalid_version_display_contains_id_and_version() {
    let err = SnapshotError::InvalidVersion {
        aggregate_id: "order-9".to_string(),
        version: 0,
    };
    let msg = err.to_string();
    assert!(msg.contains("order-9"));
    assert!(msg.contains("0"));
}

/// @covers: SnapshotError
#[test]
fn test_snapshot_error_unavailable_display_contains_detail() {
    let err = SnapshotError::Unavailable("connection refused".to_string());
    assert!(err.to_string().contains("connection refused"));
}

/// @covers: SnapshotError
#[test]
fn test_snapshot_error_variants_are_distinct() {
    let a = SnapshotError::Unavailable("x".to_string());
    let b = SnapshotError::Internal("x".to_string());
    assert_ne!(a, b);
}
