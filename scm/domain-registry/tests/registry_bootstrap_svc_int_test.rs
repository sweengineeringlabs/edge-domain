//! Rule-222 coverage for [`RegistryBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{BootstrapNameRequest, RegistryBootstrap, StdRegistryFactory};

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdRegistryFactory;
    assert!(
        !f.bootstrap_name(BootstrapNameRequest).unwrap().name.is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_expected_value_error() {
    let f = StdRegistryFactory;
    assert_eq!(
        f.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "registry",
        "bootstrap_name must return the expected static value"
    );
}

/// @covers: RegistryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn RegistryBootstrap = &StdRegistryFactory;
    let _ = f.bootstrap_name(BootstrapNameRequest);
}
