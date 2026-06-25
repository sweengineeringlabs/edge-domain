//! Integration tests — `StdRegistryBridge` (api type).

use std::sync::Arc;

use edge_domain_handler::{HandlerRegistry, InProcessHandlerRegistry, RegistryBridge, StdRegistryBridge};
use edge_domain_service::{Service, ServiceError, ServiceRegistry, ServiceRegistryTrait};
use futures::future::BoxFuture;

struct EchoSvc;

impl Service for EchoSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str { "echo" }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

/// @covers: StdRegistryBridge — bridge transfers services and returns correct count
#[test]
fn test_std_registry_bridge_bridge_transfers_service_happy() {
    let src = ServiceRegistry::<String, String>::default();
    src.register(Arc::new(EchoSvc));
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let count = StdRegistryBridge.bridge(&src, &dst);

    assert_eq!(count, 1);
    assert_eq!(dst.len(), 1);
    assert_eq!(dst.list_ids(), vec!["echo"]);
}

/// @covers: StdRegistryBridge — bridge on empty source never errors; returns 0
#[test]
fn test_std_registry_bridge_bridge_empty_source_never_errors_error() {
    let src = ServiceRegistry::<String, String>::default();
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let count = StdRegistryBridge.bridge(&src, &dst);

    assert_eq!(count, 0);
    assert_eq!(dst.len(), 0);
}

/// @covers: StdRegistryBridge — is Copy; two copies produce consistent results
#[test]
fn test_std_registry_bridge_is_copy_edge() {
    let b = StdRegistryBridge;
    let copy = b;

    let src = ServiceRegistry::<String, String>::default();
    src.register(Arc::new(EchoSvc));
    let dst = InProcessHandlerRegistry::<String, String>::default();

    let c1 = b.bridge(&src, &dst);
    assert_eq!(c1, 1);

    let src2 = ServiceRegistry::<String, String>::default();
    let dst2 = InProcessHandlerRegistry::<String, String>::default();
    let c2 = copy.bridge(&src2, &dst2);
    assert_eq!(c2, 0);
}
