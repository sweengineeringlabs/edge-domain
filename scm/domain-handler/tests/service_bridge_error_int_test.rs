//! Integration tests — `ServiceError` → `HandlerError` mapping contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::NoopCommandBus;
use edge_domain_handler::{Handler, HandlerContext, IntoHandler, Validator};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

fn make_ctx<'a>(
    security: &'a SecurityContext,
    observer: &'a dyn edge_domain_observer::ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext { security, commands: &NoopCommandBus, observer }
}

struct InvalidRequestService;
impl Service for InvalidRequestService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "invalid.service"
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Err(ServiceError::InvalidRequest("bad input".into())) })
    }
}

struct NotFoundService;
impl Service for NotFoundService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "notfound.service"
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Err(ServiceError::NotFound("gone".into())) })
    }
}

struct UnnamedService;
impl Service for UnnamedService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        ""
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: From<ServiceError> for HandlerError
#[tokio::test]
async fn test_invalid_request_service_error_maps_to_handler_error_happy() {
    let h = IntoHandler::into_handler(InvalidRequestService);
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, observer.as_ref());
    let err = h.execute("x".into(), ctx).await.unwrap_err();
    assert!(err.to_string().contains("bad input"));
}

/// @covers: From<ServiceError> for HandlerError
#[tokio::test]
async fn test_not_found_service_error_maps_to_handler_error_error() {
    let h = IntoHandler::into_handler(NotFoundService);
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = make_ctx(&security, observer.as_ref());
    let err = h.execute("x".into(), ctx).await.unwrap_err();
    assert!(err.to_string().contains("gone"));
}

/// @covers: Validator (via empty name)
#[test]
fn test_empty_service_name_produces_handler_error_edge() {
    let h = IntoHandler::into_handler(UnnamedService);
    let err = h.validate().unwrap_err();
    assert!(err.to_string().contains("empty"));
}
