//! Integration tests — [`IntoHandler`] SAF surface (`into_handler_svc`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    IntoHandler, IntoHandlerRequest, Validator, ValidatorRequest, INTO_HANDLER_SVC,
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
            name: "echo".to_string(),
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

/// @covers: INTO_HANDLER_SVC
#[test]
fn test_into_handler_svc_constant_value_is_stable_happy() {
    assert_eq!(INTO_HANDLER_SVC, "into_handler");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_trait_accessible_via_saf_surface_happy() {
    use edge_application_handler::{Handler, IdRequest};
    let h = EchoSvc.into_handler(IntoHandlerRequest).unwrap().handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: IntoHandler
#[test]
fn test_into_handler_svc_empty_name_fails_validation_edge() {
    let h = EmptySvc.into_handler(IntoHandlerRequest).unwrap().handler;
    assert!(h.validate(ValidatorRequest).is_err());
}
