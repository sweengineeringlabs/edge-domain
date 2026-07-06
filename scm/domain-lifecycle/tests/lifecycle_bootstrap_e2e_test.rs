//! Rule-222 coverage for [`LifecycleBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{BootstrapNameRequest, LifecycleBootstrap, StdLifecycleFactory};

/// @covers: LifecycleBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdLifecycleFactory;
    assert!(
        !f.bootstrap_name(BootstrapNameRequest).unwrap().name.is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: LifecycleBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdLifecycleFactory;
    let name1 = f.bootstrap_name(BootstrapNameRequest).unwrap().name;
    let name2 = f.bootstrap_name(BootstrapNameRequest).unwrap().name;
    assert_eq!(name1, name2);
}

/// @covers: LifecycleBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn LifecycleBootstrap = &StdLifecycleFactory;
    let _ = f.bootstrap_name(BootstrapNameRequest).unwrap();
}
