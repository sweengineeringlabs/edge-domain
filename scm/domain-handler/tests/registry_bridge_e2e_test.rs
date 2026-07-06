//! End-to-end contract tests for the `RegistryBridge` trait, exercised through the
//! crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_handler::{
    BridgeRequest, EmptinessRequest, HandlerRegistry, InProcessHandlerRegistry, LenRequest,
    RegistryBridge, StdRegistryBridge,
};
use edge_domain_service::{
    NameRequest, NameResponse, RegisterServiceRequest, Service, ServiceError,
    ServiceRegistry as ServiceRegistryTrait, ServiceRegistryStore,
};
use futures::future::BoxFuture;

struct GreetSvc;
impl Service for GreetSvc {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "greet".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: RegistryBridge::bridge
#[test]
fn test_bridge_transfers_services_into_handler_registry_happy() {
    let src = ServiceRegistryStore::<String, String>::default();
    src.register(&RegisterServiceRequest::new(Arc::new(GreetSvc)))
        .unwrap();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 1);
    assert_eq!(dst.len(LenRequest).unwrap().count, 1);
}

/// @covers: RegistryBridge::bridge
#[test]
fn test_bridge_empty_source_transfers_nothing_error() {
    let src = ServiceRegistryStore::<String, String>::default();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 0);
    assert!(dst.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: RegistryBridge::default_bridge
#[test]
fn test_default_bridge_returns_functional_bridge_edge() {
    let src = ServiceRegistryStore::<String, String>::default();
    src.register(&RegisterServiceRequest::new(Arc::new(GreetSvc)))
        .unwrap();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let bridge = StdRegistryBridge::default_bridge();
    let result = bridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 1);
}
