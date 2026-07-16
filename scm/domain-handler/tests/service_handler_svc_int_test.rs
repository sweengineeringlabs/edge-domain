//! Integration tests — [`ServiceHandler`] marker trait via [`IntoHandler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Handler, IdRequest, IntoHandler, IntoHandlerRequest, ServiceHandler, Validator,
    ValidatorRequest, BRIDGE_CONTEXT,
};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoSvc;
impl Service for EchoSvc {
    type Request = TextPayload;
    type Response = TextPayload;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo.svc".to_string(),
        })
    }
    fn execute(&self, req: TextPayload) -> BoxFuture<'_, Result<TextPayload, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct EmptySvc;
impl Service for EmptySvc {
    type Request = TextPayload;
    type Response = TextPayload;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: String::new(),
        })
    }
    fn execute(&self, req: TextPayload) -> BoxFuture<'_, Result<TextPayload, ServiceError>> {
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
