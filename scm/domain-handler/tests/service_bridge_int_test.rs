//! Integration tests — [`ServiceBridge`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{Handler, IdRequest, IntoHandler, IntoHandlerRequest, ServiceBridge};
use edge_application_service::{NameRequest, NameResponse, Service, ServiceError};
use futures::future::BoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoService;
impl Service for EchoService {
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
