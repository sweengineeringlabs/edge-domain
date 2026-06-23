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
    let name1 = f.bootstrap_name();
    let name2 = f.bootstrap_name();
    assert_eq!(name1, name2);
}

/// @covers: SnapshotStoreBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn SnapshotStoreBootstrap = &StdSnapshotStoreFactory;
    let _ = f.bootstrap_name();
}
