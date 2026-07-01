//! Integration tests — `EchoHandler` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::marker::PhantomData;
use std::sync::Arc;

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    EchoHandler, ExecutionRequest, Handler, HandlerContext, HealthCheckRequest, IdRequest,
    PatternRequest,
};
use edge_domain_observer::{ObserverContext, StdObserveFactory};
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
use futures::executor::block_on;

fn unauth_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn edge_domain_command::CommandBus,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

/// @covers: EchoHandler::execute — returns request unchanged
#[test]
fn test_execute_returns_request_unchanged_happy() {
    let h = EchoHandler::<String>::from(("echo", "/"));
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: "hello".into(),
            ctx: &ctx
        }))
        .unwrap(),
        "hello"
    );
}

/// @covers: EchoHandler::id — returns configured id
#[test]
fn test_id_returns_configured_id_happy() {
    let h = EchoHandler::<String>::from(("my-echo", "/*"));
    assert_eq!(h.id(IdRequest).unwrap().id, "my-echo");
}

/// @covers: EchoHandler::pattern — returns configured pattern
#[test]
fn test_pattern_returns_configured_pattern_happy() {
    let h = EchoHandler::<String>::from(("e", "/path"));
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/path");
}

/// @covers: EchoHandler::execute — empty string returns empty string
#[test]
fn test_execute_empty_string_returns_empty_string_edge() {
    let h = EchoHandler::<String>::from(("e", "/"));
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: "".into(),
            ctx: &ctx
        }))
        .unwrap(),
        ""
    );
}

/// @covers: EchoHandler::health_check default
#[test]
fn test_health_check_returns_true_happy() {
    let h = EchoHandler::<String>::from(("e", "/"));
    assert!(
        block_on(h.health_check(HealthCheckRequest))
            .unwrap()
            .healthy
    );
}

/// @covers: EchoHandler::execute — context is accepted and ignored (echo never inspects it)
#[test]
fn test_execute_with_security_context_returns_same_value_happy() {
    let h = EchoHandler::<String>::from(("e", "/"));
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: "world".into(),
            ctx: &ctx
        }))
        .unwrap(),
        "world"
    );
}

/// @covers: EchoHandler — usable as dyn Handler
#[test]
fn test_echo_handler_usable_as_dyn_handler_edge() {
    let h: Arc<dyn Handler<Request = String, Response = String>> = Arc::new(EchoHandler {
        id: "dyn".into(),
        pattern: "/".into(),
        _marker: PhantomData,
    });
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = unauth_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(h.execute(ExecutionRequest {
            req: "dyn-test".into(),
            ctx: &ctx
        }))
        .unwrap(),
        "dyn-test"
    );
}
