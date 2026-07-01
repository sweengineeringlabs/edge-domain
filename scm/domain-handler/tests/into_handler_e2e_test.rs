//! End-to-end contract tests for the `IntoHandler` trait, exercised through a
//! test-double `Service` implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::NoopCommandBus;
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, IdRequest, IntoHandler, IntoHandlerRequest,
    Validator, ValidatorRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

fn make_ctx<'a>(
    security: &'a SecurityContext,
    observer: &'a dyn edge_domain_observer::ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: &NoopCommandBus,
        observer,
    }
}

struct GreetService;
impl Service for GreetService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "greet".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(format!("hello {req}")) })
    }
}

/// @covers: IntoHandler::into_handler
#[test]
fn test_into_handler_id_matches_service_name_happy() {
    let h = GreetService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "greet");
}

/// @covers: IntoHandler::into_handler
#[test]
fn test_into_handler_produces_valid_handler_edge() {
    let h = GreetService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.validate(ValidatorRequest), Ok(()));
}

/// @covers: IntoHandler::into_handler
#[tokio::test]
async fn test_into_handler_execute_delegates_to_service_happy() {
    let handler = GreetService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, observer.as_ref());
    let result = handler
        .execute(ExecutionRequest {
            req: "world".into(),
            ctx: &ctx,
        })
        .await;
    assert_eq!(result.unwrap(), "hello world");
}

/// @covers: IntoHandler::into_handler
#[tokio::test]
async fn test_into_handler_execute_propagates_service_error_error() {
    struct FailingService;
    impl Service for FailingService {
        type Request = String;
        type Response = String;
        fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
            Ok(NameResponse {
                name: "failing".to_string(),
            })
        }
        fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
            Box::pin(async move { Err(ServiceError::Internal("boom".into())) })
        }
    }

    let handler = FailingService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, observer.as_ref());
    let result = handler
        .execute(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        })
        .await;
    assert!(result.is_err());
}
