//! Integration tests — `EchoHandler` type.

use std::sync::Arc;

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{EchoHandler, Handler, HandlerContext};
use edge_domain_observer::{ObserveContext, StdObserveFactory};
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

fn unauth_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn edge_domain_command::CommandBus,
    observer: &'a dyn ObserveContext,
) -> HandlerContext<'a> {
    HandlerContext::new(security, bus, observer)
}

/// @covers: EchoHandler::execute — returns request unchanged
#[test]
fn test_execute_returns_request_unchanged_happy() {
    let h = EchoHandler::<String>::new("echo", "/");
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(block_on(h.execute("hello".into(), ctx)).unwrap(), "hello");
}

/// @covers: EchoHandler::id — returns configured id
#[test]
fn test_id_returns_configured_id_happy() {
    let h = EchoHandler::<String>::new("my-echo", "/*");
    assert_eq!(h.id(), "my-echo");
}

/// @covers: EchoHandler::pattern — returns configured pattern
#[test]
fn test_pattern_returns_configured_pattern_happy() {
    let h = EchoHandler::<String>::new("e", "/path");
    assert_eq!(h.pattern(), "/path");
}

/// @covers: EchoHandler::execute — empty string returns empty string
#[test]
fn test_execute_empty_string_returns_empty_string_edge() {
    let h = EchoHandler::<String>::new("e", "/");
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(block_on(h.execute("".into(), ctx)).unwrap(), "");
}

/// @covers: EchoHandler::health_check default
#[test]
fn test_health_check_returns_true_happy() {
    let h = EchoHandler::<String>::new("e", "/");
    assert!(block_on(h.health_check()));
}

/// @covers: EchoHandler::execute — context is accepted and ignored (echo never inspects it)
#[test]
fn test_execute_with_security_context_returns_same_value_happy() {
    let h = EchoHandler::<String>::new("e", "/");
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(block_on(h.execute("world".into(), ctx)).unwrap(), "world");
}

/// @covers: EchoHandler — usable as dyn Handler
#[test]
fn test_echo_handler_usable_as_dyn_handler_edge() {
    let h: Arc<dyn Handler<Request = String, Response = String>> =
        Arc::new(EchoHandler::new("dyn", "/"));
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(h.execute("dyn-test".into(), ctx)).unwrap(),
        "dyn-test"
    );
}
