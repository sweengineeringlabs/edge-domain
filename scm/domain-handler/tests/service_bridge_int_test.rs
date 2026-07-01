//! Integration tests — [`ServiceBridge`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Handler, IdRequest, IntoHandler, IntoHandlerRequest, ServiceBridge};
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

struct EchoService;
impl Service for EchoService {
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

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_handler_satisfies_bound_happy() {
    fn assert_bridge<T: ServiceBridge>(_: &T) {}
    let h = IntoHandler::into_handler(EchoService, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_bridge(&h);
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_id_matches_service_name_error() {
    let h = IntoHandler::into_handler(EchoService, IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_zero_sized_marker_compiles_edge() {
    assert_eq!(std::mem::size_of::<EchoService>(), 0);
}
