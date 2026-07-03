//! Integration tests for `EchoHandler` and the `echo_handler` factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, EchoHandler, Handler, HandlerContext};
use edge_domain_handler::{ExecutionRequest, HealthCheckRequest, IdRequest, PatternRequest};
use edge_domain_observer::{ObserverContext, StdObserveFactory};
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
use std::sync::Arc;

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a Arc<dyn edge_domain::CommandBus>,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus.as_ref(),
        observer,
    }
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_factory_returns_arc_handler() {
    let _: Arc<dyn Handler<Request = String, Response = String>> =
        Domain::echo_handler("id", "/path");
}

/// @covers: echo_handler
#[tokio::test]
async fn test_echo_handler_returns_request_as_response() {
    let h = Domain::echo_handler::<String>("echo", "/echo");
    let security = SecurityServices::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    let result = h
        .execute(ExecutionRequest {
            req: "hello".to_string(),
            ctx: &ctx,
        })
        .await
        .unwrap();
    assert_eq!(result, "hello");
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_id_matches_constructor_arg() {
    let h: Arc<dyn Handler<Request = String, Response = String>> =
        Domain::echo_handler("my-handler", "/api/v1");
    assert_eq!(h.id(IdRequest).unwrap().id, "my-handler");
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_pattern_matches_constructor_arg() {
    let h: Arc<dyn Handler<Request = String, Response = String>> =
        Domain::echo_handler("id", "/api/v1/things");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/api/v1/things");
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_struct_health_check_defaults_to_true() {
    let h = EchoHandler::<String>::from(("id", "/p"));
    assert!(h.health_check(HealthCheckRequest).await.unwrap().healthy);
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_works_with_numeric_type() {
    let h: Arc<dyn Handler<Request = u64, Response = u64>> = Domain::echo_handler("num", "/num");
    let security = SecurityServices::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        h.execute(ExecutionRequest {
            req: 42u64,
            ctx: &ctx
        })
        .await
        .unwrap(),
        42u64
    );
}
