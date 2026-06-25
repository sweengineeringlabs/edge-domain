//! Integration tests — [`Validator`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{IntoHandler, Validator};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct NamedService;
impl Service for NamedService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "named.service"
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct UnnamedService;
impl Service for UnnamedService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        ""
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: Validator::validate
#[test]
fn test_validate_named_service_ok_happy() {
    let h = IntoHandler::into_handler(NamedService);
    assert_eq!(h.validate(), Ok(()));
}

/// @covers: Validator::validate
#[test]
fn test_validate_empty_name_returns_error_error() {
    let h = IntoHandler::into_handler(UnnamedService);
    assert!(h.validate().is_err());
}

/// @covers: Validator::validate
#[test]
fn test_validate_error_message_describes_constraint_edge() {
    let h = IntoHandler::into_handler(UnnamedService);
    let err = h.validate().unwrap_err();
    assert!(err.to_string().contains("empty"));
}
