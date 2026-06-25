//! Integration tests — [`ServiceHandler`] marker trait via [`IntoHandler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Handler, IntoHandler, ServiceHandler, Validator, BRIDGE_CONTEXT};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct EchoSvc;
impl Service for EchoSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "echo.svc"
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct EmptySvc;
impl Service for EmptySvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        ""
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_svc_handler_satisfies_bridge_bound_happy() {
    fn assert_bridge<T: ServiceHandler>(_: &T) {}
    let h = IntoHandler::into_handler(EchoSvc);
    assert_bridge(&h);
    assert_eq!(h.id(), "echo.svc");
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_svc_empty_name_fails_validate_error() {
    let h = IntoHandler::into_handler(EmptySvc);
    assert!(h.validate().is_err());
}

/// @covers: BRIDGE_CONTEXT
#[test]
fn test_service_handler_svc_bridge_context_constant_edge() {
    assert_eq!(BRIDGE_CONTEXT, "service_handler");
}
