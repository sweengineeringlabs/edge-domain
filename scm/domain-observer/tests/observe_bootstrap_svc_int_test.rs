//! Rule-222 coverage for [`ObserveBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_observer::{ObserveBootstrap, StdObserveFactory};

/// @covers: ObserveBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdObserveFactory;
    assert!(
        !f.bootstrap_name().is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: ObserveBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdObserveFactory;
    assert_eq!(
        f.bootstrap_name(),
        f.bootstrap_name(),
        "bootstrap_name must return the same value on repeated calls"
    );
}

/// @covers: ObserveBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn ObserveBootstrap = &StdObserveFactory;
    let _ = f.bootstrap_name();
}
