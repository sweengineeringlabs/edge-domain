//! End-to-end contract tests for the `Handler` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, HealthCheckRequest,
    HealthCheckResponse, IdRequest, PatternRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};

struct EchoHandlerDouble;

#[async_trait::async_trait]
impl Handler for EchoHandlerDouble {
    type Request = String;
    type Response = String;

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Ok(req.req)
    }
}

struct FailingHandlerDouble;

#[async_trait::async_trait]
impl Handler for FailingHandlerDouble {
    type Request = String;
    type Response = String;

    async fn execute(&self, _req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("boom".into()))
    }

    async fn health_check(
        &self,
        _req: HealthCheckRequest,
    ) -> Result<HealthCheckResponse, HandlerError> {
        Ok(HealthCheckResponse { healthy: false })
    }
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_execute_echo_returns_input_happy() {
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = EchoHandlerDouble
        .execute(ExecutionRequest {
            req: "payload".into(),
            ctx: &ctx,
        })
        .await;
    assert_eq!(result.unwrap(), "payload");
}

/// @covers: Handler::execute
#[tokio::test]
async fn test_execute_failing_handler_returns_execution_failed_error() {
    let security = SecurityServices::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = FailingHandlerDouble
        .execute(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        })
        .await;
    assert!(matches!(result, Err(HandlerError::ExecutionFailed(_))));
}

/// @covers: Handler::id
#[test]
fn test_id_default_returns_handler_edge() {
    assert_eq!(EchoHandlerDouble.id(IdRequest).unwrap().id, "handler");
}

/// @covers: Handler::pattern
#[test]
fn test_pattern_default_returns_empty_string_edge() {
    assert_eq!(
        EchoHandlerDouble.pattern(PatternRequest).unwrap().pattern,
        ""
    );
}

/// @covers: Handler::health_check
#[tokio::test]
async fn test_health_check_default_returns_healthy_happy() {
    assert!(
        EchoHandlerDouble
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}

/// @covers: Handler::health_check
#[tokio::test]
async fn test_health_check_overridden_returns_unhealthy_error() {
    assert!(
        !FailingHandlerDouble
            .health_check(HealthCheckRequest)
            .await
            .unwrap()
            .healthy
    );
}
