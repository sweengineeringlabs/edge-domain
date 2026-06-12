//! Integration tests — `Handler` trait via SAF facade.

use async_trait::async_trait;
use edge_domain_handler::{Handler, HandlerError};
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

struct OkHandler;

#[async_trait]
impl Handler<String, String> for OkHandler {
    fn id(&self) -> &str {
        "ok-handler"
    }
    fn pattern(&self) -> &str {
        "/ok"
    }
    async fn execute(&self, req: String) -> Result<String, HandlerError> {
        Ok(req.to_uppercase())
    }
}

struct FailHandler;

#[async_trait]
impl Handler<String, String> for FailHandler {
    async fn execute(&self, _req: String) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("deliberate".into()))
    }
}

struct UnhealthyHandler;

#[async_trait]
impl Handler<String, String> for UnhealthyHandler {
    async fn execute(&self, _req: String) -> Result<String, HandlerError> {
        Err(HandlerError::Unhealthy)
    }
    async fn health_check(&self) -> bool {
        false
    }
}

/// @covers: Handler::execute — success path
#[test]
fn test_execute_ok_handler_returns_response_happy() {
    let result = block_on(OkHandler.execute("hello".into()));
    assert_eq!(result.unwrap(), "HELLO");
}

/// @covers: Handler::execute — error propagation
#[test]
fn test_execute_failing_handler_returns_err_error() {
    let result = block_on(FailHandler.execute("x".into()));
    assert!(result.is_err());
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

/// @covers: Handler::execute_with_context delegates to execute
#[test]
fn test_execute_with_context_delegates_to_execute_happy() {
    let ctx = SecurityContext::unauthenticated();
    let result = block_on(OkHandler.execute_with_context("world".into(), ctx));
    assert_eq!(result.unwrap(), "WORLD");
}

/// @covers: Handler::execute_with_context with authenticated context
#[test]
fn test_execute_with_context_authenticated_context_still_executes_edge() {
    use edge_domain_security::AnonymousPrincipal;
    let ctx = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    let result = block_on(OkHandler.execute_with_context("test".into(), ctx));
    assert_eq!(result.unwrap(), "TEST");
}

/// @covers: Handler::id — non-overriding impl always returns default (no error path)
#[test]
fn test_id_non_overriding_impl_returns_default_handler_error() {
    // FailHandler does not override id(); the default "handler" is always returned.
    // This tests that the absence of an override does not produce an error.
    assert_eq!(FailHandler.id(), "handler");
}

/// @covers: Handler::pattern — non-overriding impl always returns empty (no error path)
#[test]
fn test_pattern_non_overriding_impl_returns_empty_string_error() {
    // FailHandler does not override pattern(); the default "" is always returned.
    assert_eq!(FailHandler.pattern(), "");
}
