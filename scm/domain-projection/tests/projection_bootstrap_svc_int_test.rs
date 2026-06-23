//! Rule-222 coverage for [`ProjectionBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_projection::{ProjectionBootstrap, StdProjectionFactory};

/// @covers: ProjectionBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdProjectionFactory;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: ProjectionBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdProjectionFactory;
    let name1 = f.bootstrap_name();
    let name2 = f.bootstrap_name();
    assert_eq!(
        name1,
        name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(name1, "StdProjectionFactory", "bootstrap_name must return expected identifier");
}

/// @covers: ProjectionBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn ProjectionBootstrap = &StdProjectionFactory;
    let _ = f.bootstrap_name();
}
