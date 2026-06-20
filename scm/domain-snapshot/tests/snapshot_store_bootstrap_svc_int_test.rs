//! Rule-222 coverage for [`SnapshotStoreBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_snapshot::{SnapshotStoreBootstrap, StdSnapshotStoreFactory};

/// @covers: SnapshotStoreBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdSnapshotStoreFactory;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: SnapshotStoreBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdSnapshotStoreFactory;
    assert_eq!(
        f.bootstrap_name(),
        f.bootstrap_name(),
        "bootstrap_name must return the same value on repeated calls"
    );
}

/// @covers: SnapshotStoreBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn SnapshotStoreBootstrap = &StdSnapshotStoreFactory;
    let _ = f.bootstrap_name();
}
