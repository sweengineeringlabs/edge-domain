//! Rule-222 coverage for [`PolicyBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_policy::{BootstrapNameRequest, PolicyBootstrap, StdPolicyFactory};

/// @covers: PolicyBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdPolicyFactory;
    assert!(
        !f.bootstrap_name(BootstrapNameRequest).unwrap().name.is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: PolicyBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdPolicyFactory;
    let name1 = f.bootstrap_name(BootstrapNameRequest).unwrap().name;
    let name2 = f.bootstrap_name(BootstrapNameRequest).unwrap().name;
    assert_eq!(
        name1,
        name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(name1, "policy", "bootstrap_name must return expected identifier");
}

/// @covers: PolicyBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn PolicyBootstrap = &StdPolicyFactory;
    let _ = f.bootstrap_name(BootstrapNameRequest).unwrap();
}
