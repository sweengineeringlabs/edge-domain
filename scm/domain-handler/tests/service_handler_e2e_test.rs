//! Integration tests — [`ServiceHandler`] constructed via [`IntoHandler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Handler, IdRequest, IntoHandler, IntoHandlerRequest, Validator, ValidatorRequest,
};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

struct NamedSvc;
impl Service for NamedSvc {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "svc.named".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct UnnamedSvc;
impl Service for UnnamedSvc {
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
fn test_service_handler_stores_service_name_as_id_happy() {
    let h = IntoHandler::into_handler(NamedSvc, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "svc.named");
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_empty_name_fails_validation_error() {
    let h = IntoHandler::into_handler(UnnamedSvc, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert!(h.validate(ValidatorRequest).is_err());
}

/// @covers: ServiceHandler
#[test]
fn test_service_handler_nonempty_name_passes_validation_edge() {
    let h = IntoHandler::into_handler(NamedSvc, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.validate(ValidatorRequest), Ok(()));
}
