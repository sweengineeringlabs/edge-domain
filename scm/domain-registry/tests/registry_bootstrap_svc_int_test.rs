//! Rule-222 coverage for [`RegistryBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{RegistryBootstrap, StdRegistryFactory};

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdRegistryFactory;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdRegistryFactory;
    let name1 = f.bootstrap_name();
    let name2 = f.bootstrap_name();
    assert_eq!(
        name1,
        name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(name1, "StdRegistryFactory", "bootstrap_name must return expected identifier");
}

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn RegistryBootstrap = &StdRegistryFactory;
    let _ = f.bootstrap_name();
}
