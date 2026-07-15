//! Integration tests — [`ServiceHandler`] marker trait via [`IntoHandler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Handler, IdRequest, IntoHandler, IntoHandlerRequest, ServiceHandler, Validator,
    ValidatorRequest, BRIDGE_CONTEXT,
};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

struct EchoSvc;
impl Service for EchoSvc {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo.svc".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct EmptySvc;
impl Service for EmptySvc {
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

/// @covers: ServiceHandler
#[test]
fn test_service_handler_svc_handler_satisfies_bridge_bound_happy() {
    fn assert_bridge<T: ServiceHandler>(_: &T) {}
    let h = IntoHandler::into_handler(EchoSvc, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_bridge(&h);
    assert_eq!(h.id(IdRequest).unwrap().id, "echo.svc");
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_svc_empty_name_fails_validate_error() {
    let h = IntoHandler::into_handler(EmptySvc, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert!(h.validate(ValidatorRequest).is_err());
}

/// @covers: BRIDGE_CONTEXT
#[test]
fn test_service_handler_svc_bridge_context_constant_edge() {
    assert_eq!(BRIDGE_CONTEXT, "service_handler");
}
