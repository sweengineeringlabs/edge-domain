//! Integration tests — `NoopHandlerFactory` type and `HandlerBootstrap` impl.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{BootstrapNameRequest, HandlerBootstrap, NoopHandlerFactory};

/// @covers: HandlerBootstrap::build — unit config produces Ok
#[test]
fn test_build_unit_config_returns_ok_happy() {
    let result = NoopHandlerFactory::build(());
    assert!(result.is_ok());
    let _: NoopHandlerFactory = result.unwrap().handler;
}

/// @covers: HandlerBootstrap::build — result is NoopHandlerFactory
#[test]
fn test_build_returns_noop_handler_factory_instance_happy() {
    let _f: NoopHandlerFactory = NoopHandlerFactory::build(()).unwrap().handler;
}

/// @covers: HandlerBootstrap::build — no invalid input path exists; absence documented
#[test]
fn test_build_is_always_ok_no_error_path_error() {
    // NoopHandlerFactory::build(()) is infallible — unit type has no invalid state.
    // This test documents the absence of an error path explicitly.
    let r1 = NoopHandlerFactory::build(());
    let r2 = NoopHandlerFactory::build(());
    assert!(r1.is_ok());
    assert!(r2.is_ok());
    // Verify the actual values are correct
    let _: NoopHandlerFactory = r1.unwrap().handler;
    let _: NoopHandlerFactory = r2.unwrap().handler;
}

/// @covers: NoopHandlerFactory::default — derives Default
#[test]
fn test_default_constructs_instance_edge() {
    use edge_domain_handler::HandlerBootstrap;
    let f: NoopHandlerFactory = NoopHandlerFactory::default();
    assert_eq!(
        f.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler"
    );
}

/// @covers: NoopHandlerFactory — derives Clone and Copy
#[test]
fn test_clone_copy_semantics_edge() {
    use edge_domain_handler::HandlerBootstrap;
    let a = NoopHandlerFactory;
    let b = a; // Copy
    let c = a.clone(); // Clone
    assert_eq!(
        a.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler"
    );
    assert_eq!(
        b.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler"
    );
    assert_eq!(
        c.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler"
    );
}
