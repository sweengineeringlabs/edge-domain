//! Integration tests — [`IntoHandler`] SAF surface (`into_handler_svc`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    IntoHandler, IntoHandlerRequest, Validator, ValidatorRequest, INTO_HANDLER_SVC,
};
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

struct EchoSvc;
impl Service for EchoSvc {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo".to_string(),
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

/// @covers: INTO_HANDLER_SVC
#[test]
fn test_into_handler_svc_constant_value_is_stable_happy() {
    assert_eq!(INTO_HANDLER_SVC, "into_handler");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_trait_accessible_via_saf_surface_happy() {
    use edge_domain_handler::{Handler, IdRequest};
    let h = EchoSvc.into_handler(IntoHandlerRequest).unwrap().handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_empty_name_fails_validation_edge() {
    let h = EmptySvc.into_handler(IntoHandlerRequest).unwrap().handler;
    assert!(h.validate(ValidatorRequest).is_err());
}
