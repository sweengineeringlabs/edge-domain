//! Integration tests — `NoopHandlerFactory` type and `HandlerFactory` impl.

use edge_domain_handler::{HandlerFactory, NoopHandlerFactory};

/// @covers: HandlerFactory::build — unit config produces Ok
#[test]
fn test_build_unit_config_returns_ok_happy() {
    assert!(NoopHandlerFactory::build(()).is_ok());
}

/// @covers: HandlerFactory::build — result is NoopHandlerFactory
#[test]
fn test_build_returns_noop_handler_factory_instance_happy() {
    let _f: NoopHandlerFactory = NoopHandlerFactory::build(()).unwrap();
}

/// @covers: HandlerFactory::build — no invalid input path exists; absence documented
#[test]
fn test_build_is_always_ok_no_error_path_error() {
    // NoopHandlerFactory::build(()) is infallible — unit type has no invalid state.
    // This test documents the absence of an error path explicitly.
    let r1 = NoopHandlerFactory::build(());
    let r2 = NoopHandlerFactory::build(());
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}

/// @covers: NoopHandlerFactory::default — derives Default
#[test]
fn test_default_constructs_instance_edge() {
    let _f: NoopHandlerFactory = NoopHandlerFactory::default();
}

/// @covers: NoopHandlerFactory — derives Clone and Copy
#[test]
fn test_clone_copy_semantics_edge() {
    let a = NoopHandlerFactory;
    let b = a;
    let _c = a.clone();
    let _ = (a, b);
}
