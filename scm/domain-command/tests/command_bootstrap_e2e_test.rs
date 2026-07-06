//! Rule-222 coverage for [`CommandBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{BootstrapNameRequest, CommandBootstrap, StdCommandBusFactory};

/// @covers: CommandBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdCommandBusFactory;
    let response = f.bootstrap_name(BootstrapNameRequest).unwrap();
    assert!(
        !response.name.is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: CommandBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdCommandBusFactory;
    let response1 = f.bootstrap_name(BootstrapNameRequest).unwrap();
    let response2 = f.bootstrap_name(BootstrapNameRequest).unwrap();
    assert_eq!(
        response1.name, response2.name,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(
        response1.name, "command",
        "bootstrap_name must return expected identifier"
    );
}

/// @covers: CommandBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn CommandBootstrap = &StdCommandBusFactory;
    let _ = f.bootstrap_name(BootstrapNameRequest).unwrap();
}
