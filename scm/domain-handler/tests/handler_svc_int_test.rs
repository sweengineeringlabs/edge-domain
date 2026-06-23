//! Integration tests — `Handler` trait via SAF facade.

use async_trait::async_trait;
use edge_domain_command::{CommandBus, CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_observer::{ObserveContext, StdObserveFactory};
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

struct OkHandler;

#[async_trait]
impl Handler for OkHandler {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "ok-handler"
    }
    fn pattern(&self) -> &str {
        "/ok"
    }

    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(req.to_uppercase())
    }
}

struct FailHandler;

#[async_trait]
impl Handler for FailHandler {
    type Request = String;
    type Response = String;

    async fn execute(
        &self,
        _req: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("deliberate".into()))
    }
}

struct UnhealthyHandler;

#[async_trait]
impl Handler for UnhealthyHandler {
    type Request = String;
    type Response = String;

    async fn execute(
        &self,
        _req: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        Err(HandlerError::Unhealthy)
    }
    async fn health_check(&self) -> bool {
        false
    }
}

fn make_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn CommandBus,
    observer: &'a dyn ObserveContext,
) -> HandlerContext<'a> {
    HandlerContext::new(security, bus, observer)
}

/// @covers: Handler::execute — success path
#[test]
fn test_execute_ok_handler_returns_response_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute("hello".into(), ctx)).unwrap(),
        "HELLO"
    );
}

/// @covers: Handler::execute — error propagation
#[test]
fn test_execute_failing_handler_returns_err_error() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert!(block_on(FailHandler.execute("x".into(), ctx)).is_err());
}

/// @covers: Handler::id default and override
#[test]
fn test_id_returns_configured_value_happy() {
    assert_eq!(OkHandler.id(), "ok-handler");
}

/// @covers: Handler::id default
#[test]
fn test_id_default_returns_handler_edge() {
    assert_eq!(FailHandler.id(), "handler");
}

/// @covers: Handler::pattern override
#[test]
fn test_pattern_returns_configured_value_happy() {
    assert_eq!(OkHandler.pattern(), "/ok");
}

/// @covers: Handler::pattern default
#[test]
fn test_pattern_default_returns_empty_edge() {
    assert_eq!(FailHandler.pattern(), "");
}

/// @covers: Handler::health_check default returns true
#[test]
fn test_health_check_default_returns_true_happy() {
    assert!(block_on(OkHandler.health_check()));
}

/// @covers: Handler::health_check overridden to false
#[test]
fn test_health_check_unhealthy_handler_returns_false_error() {
    assert!(!block_on(UnhealthyHandler.health_check()));
}

/// @covers: Handler::execute — unauthenticated context threads through correctly
#[test]
fn test_execute_with_unauthenticated_context_returns_response_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute("world".into(), ctx)).unwrap(),
        "WORLD"
    );
}

/// @covers: Handler::execute — authenticated context threads through correctly
#[test]
fn test_execute_with_authenticated_context_still_executes_edge() {
    use edge_domain_security::AnonymousPrincipal;
    let security = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = make_ctx(&security, &bus, observer.as_ref());
    assert_eq!(
        block_on(OkHandler.execute("test".into(), ctx)).unwrap(),
        "TEST"
    );
}

/// @covers: Handler::id — non-overriding impl always returns default (no error path)
#[test]
fn test_id_non_overriding_impl_returns_default_handler_error() {
    assert_eq!(FailHandler.id(), "handler");
}

/// @covers: Handler::pattern — non-overriding impl always returns empty (no error path)
#[test]
fn test_pattern_non_overriding_impl_returns_empty_string_error() {
    assert_eq!(FailHandler.pattern(), "");
}
