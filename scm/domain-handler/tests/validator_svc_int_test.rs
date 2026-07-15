//! Integration tests — [`Validator`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{IntoHandler, IntoHandlerRequest, Validator, ValidatorRequest};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

struct NamedService;
impl Service for NamedService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "named.service".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
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

/// @covers: Validator::validate
#[test]
fn test_validate_named_service_ok_happy() {
    let h = IntoHandler::into_handler(NamedService, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.validate(ValidatorRequest), Ok(()));
}

/// @covers: Validator::validate
#[test]
fn test_validate_empty_name_returns_error_error() {
    let h = IntoHandler::into_handler(UnnamedService, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert!(h.validate(ValidatorRequest).is_err());
}

/// @covers: Validator::validate
#[test]
fn test_validate_error_message_describes_constraint_edge() {
    let h = IntoHandler::into_handler(UnnamedService, IntoHandlerRequest)
        .unwrap()
        .handler;
    let err = h.validate(ValidatorRequest).unwrap_err();
    assert!(err.to_string().contains("empty"));
}
