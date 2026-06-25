//! Integration tests — [`IntoHandler`] SAF surface (`into_handler_svc`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{IntoHandler, Validator, INTO_HANDLER_SVC};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct EchoSvc;
impl Service for EchoSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "echo"
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

/// @covers: INTO_HANDLER_SVC
#[test]
fn test_into_handler_svc_constant_value_is_stable_happy() {
    assert_eq!(INTO_HANDLER_SVC, "into_handler");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_trait_accessible_via_saf_surface_happy() {
    use edge_domain_handler::Handler;
    let h = EchoSvc.into_handler();
    assert_eq!(h.id(), "echo");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_empty_name_fails_validation_edge() {
    let h = EmptySvc.into_handler();
    assert!(h.validate().is_err());
}
