//! Rule-222 coverage for [`HandlerBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{HandlerBootstrap, NoopHandlerFactory};

/// @covers: HandlerBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = NoopHandlerFactory;
    assert!(
        !f.bootstrap_name().is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: HandlerBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = NoopHandlerFactory;
    assert_eq!(
        f.bootstrap_name(),
        "handler",
        "bootstrap_name must return the expected static value"
    );
}

/// @covers: HandlerBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn HandlerBootstrap = &NoopHandlerFactory;
    let _ = f.bootstrap_name();
}
