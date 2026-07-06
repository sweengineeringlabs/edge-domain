//! Integration tests — `StdRegistryBridge` (api type).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_handler::{
    BridgeRequest, HandlerRegistry, InProcessHandlerRegistry, LenRequest, ListIdsRequest,
    RegistryBridge, StdRegistryBridge,
};
use edge_domain_service::{
    NameRequest, NameResponse, RegisterServiceRequest, Service, ServiceError,
    ServiceRegistry as ServiceRegistryTrait, ServiceRegistryStore,
};
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

/// @covers: StdRegistryBridge — bridge transfers services and returns correct count
#[test]
fn test_std_registry_bridge_bridge_transfers_service_happy() {
    let src: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    src.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 1);
    assert_eq!(dst.len(LenRequest).unwrap().count, 1);
    assert_eq!(dst.list_ids(ListIdsRequest).unwrap().ids, vec!["echo"]);
}

/// @covers: StdRegistryBridge — bridge on empty source never errors; returns 0
#[test]
fn test_std_registry_bridge_bridge_empty_source_never_errors_error() {
    let src: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 0);
    assert_eq!(dst.len(LenRequest).unwrap().count, 0);
}

/// @covers: StdRegistryBridge — is Copy; two copies produce consistent results
#[test]
fn test_std_registry_bridge_is_copy_edge() {
    let b = StdRegistryBridge;
    let copy = b;

    let src: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    src.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let c1 = b.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });
    assert_eq!(c1.unwrap().transferred, 1);

    let src2: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    let dst2 = InProcessHandlerRegistry::<String, String>::default();
    let c2 = copy.bridge(BridgeRequest {
        src: &src2,
        dst: &dst2,
    });
    assert_eq!(c2.unwrap().transferred, 0);
}
