//! Integration tests — [`ServiceHandler`] constructed via [`IntoHandler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Handler, IntoHandler, Validator};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct NamedSvc;
impl Service for NamedSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "svc.named"
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct UnnamedSvc;
impl Service for UnnamedSvc {
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
fn test_service_handler_stores_service_name_as_id_happy() {
    let h = IntoHandler::into_handler(NamedSvc);
    assert_eq!(h.id(), "svc.named");
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_empty_name_fails_validation_error() {
    let h = IntoHandler::into_handler(UnnamedSvc);
    assert!(h.validate().is_err());
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_nonempty_name_passes_validation_edge() {
    let h = IntoHandler::into_handler(NamedSvc);
    assert_eq!(h.validate(), Ok(()));
}
