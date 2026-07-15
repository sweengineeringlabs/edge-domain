//! Integration tests — `ServiceError` → `HandlerError` mapping contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_command::NoopCommandBus;
use edge_application_handler::{
    ExecutionRequest, Handler, HandlerContext, IntoHandler, IntoHandlerRequest,
    ObserverContextAdapter, Validator, ValidatorRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use edge_security_runtime::SecurityContext;
use futures::future::BoxFuture;

fn make_ctx<'a>(
    security: &'a SecurityContext,
    observer: &'a ObserverContextAdapter<'a, dyn edge_application_observer::ObserverContext>,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: &NoopCommandBus,
        observer,
    }
}

struct InvalidRequestService;
impl Service for InvalidRequestService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "invalid.service".to_string(),
        })
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Err(ServiceError::InvalidRequest("bad input".into())) })
    }
}

struct NotFoundService;
impl Service for NotFoundService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "notfound.service".to_string(),
        })
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Err(ServiceError::NotFound("gone".into())) })
    }
}

struct UnnamedService;
impl Service for UnnamedService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: String::new(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: From<ServiceError> for HandlerError
#[tokio::test]
async fn test_invalid_request_service_error_maps_to_handler_error_happy() {
    let h = IntoHandler::into_handler(InvalidRequestService, IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &observer_adapter);
    let err = h
        .execute(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        })
        .await
        .unwrap_err();
    assert!(err.to_string().contains("bad input"));
}

/// @covers: From<ServiceError> for HandlerError
#[tokio::test]
async fn test_not_found_service_error_maps_to_handler_error_error() {
    let h = IntoHandler::into_handler(NotFoundService, IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &observer_adapter);
    let err = h
        .execute(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        })
        .await
        .unwrap_err();
    assert!(err.to_string().contains("gone"));
}

/// @covers: Validator (via empty name)
#[test]
fn test_empty_service_name_produces_handler_error_edge() {
    let h = IntoHandler::into_handler(UnnamedService, IntoHandlerRequest)
        .unwrap()
        .handler;
    let err = h.validate(ValidatorRequest).unwrap_err();
    assert!(err.to_string().contains("empty"));
}
