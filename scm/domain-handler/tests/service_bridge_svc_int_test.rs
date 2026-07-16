//! Integration tests — [`ServiceBridge`] SAF re-export contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Handler, IdRequest, IntoHandler, IntoHandlerRequest, ServiceBridge, SERVICE_BRIDGE_SVC,
};
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

/// @covers: SERVICE_BRIDGE_SVC
#[test]
fn test_service_bridge_svc_identifier_is_stable_happy() {
    assert_eq!(SERVICE_BRIDGE_SVC, "service_bridge");
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_handler_satisfies_bound_happy() {
    fn assert_bridge<T: ServiceBridge>(_: &T) {}
    let h = EchoService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_bridge(&h);
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_zero_sized_marker_compiles_edge() {
    assert_eq!(std::mem::size_of::<EchoService>(), 0);
}
