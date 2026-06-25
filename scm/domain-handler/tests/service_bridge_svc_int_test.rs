//! Integration tests — [`ServiceBridge`] SAF re-export contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Handler, IntoHandler, ServiceBridge, SERVICE_BRIDGE_SVC};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct EchoService;
impl Service for EchoService {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str {
        "echo"
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
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
    let h = EchoService.into_handler();
    assert_bridge(&h);
    assert_eq!(h.id(), "echo");
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_zero_sized_marker_compiles_edge() {
    assert_eq!(std::mem::size_of::<EchoService>(), 0);
}
