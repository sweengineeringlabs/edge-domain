//! Integration tests — `RegistryBridge` trait via SAF facade.

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

struct GreetSvc;

impl Service for GreetSvc {
    type Request = String;
    type Response = String;
    fn name(&self) -> &str { "greet" }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(format!("hello {req}")) })
    }
}

struct AlternativeBridge;

impl RegistryBridge for AlternativeBridge {
    fn bridge<Req, Resp>(
        &self,
        _src: &dyn ServiceRegistryTrait<Request = Req, Response = Resp>,
        _dst: &dyn HandlerRegistry<Request = Req, Response = Resp>,
    ) -> usize
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        0
    }
}

fn make_svc_registry() -> ServiceRegistry<String, String> {
    ServiceRegistry::default()
}

fn make_handler_registry() -> InProcessHandlerRegistry<String, String> {
    InProcessHandlerRegistry::default()
}

/// @covers: RegistryBridge::bridge — all services transferred to handler registry
#[test]
fn test_bridge_transfers_all_services_happy() {
    let src = make_svc_registry();
    src.register(Arc::new(EchoSvc));
    src.register(Arc::new(GreetSvc));
    let dst = make_handler_registry();

    let count = StdRegistryBridge.bridge(&src, &dst);

    assert_eq!(count, 2);
    assert_eq!(dst.len(), 2);
    assert_eq!(dst.get("echo").unwrap().id(), "echo");
    assert_eq!(dst.get("greet").unwrap().id(), "greet");
}

/// @covers: RegistryBridge::bridge — empty source registry returns 0
#[test]
fn test_bridge_empty_registry_returns_zero_error() {
    let src = make_svc_registry();
    let dst = make_handler_registry();

    let count = StdRegistryBridge.bridge(&src, &dst);

    assert_eq!(count, 0);
    assert_eq!(dst.len(), 0);
    assert!(dst.list_ids().is_empty());
}

/// @covers: RegistryBridge::bridge — second bridge into non-empty registry accumulates
#[test]
fn test_bridge_into_non_empty_registry_accumulates_edge() {
    let src = make_svc_registry();
    src.register(Arc::new(EchoSvc));
    let dst = make_handler_registry();
    dst.register(Arc::new(
        edge_domain_handler::EchoHandler::<String>::from(("pre", "/pre")),
    ));

    let count = StdRegistryBridge.bridge(&src, &dst);

    assert_eq!(count, 1);
    assert_eq!(dst.len(), 2);
    assert!(dst.get("echo").is_some());
    assert!(dst.get("pre").is_some());
}

/// @covers: RegistryBridge::default_bridge — returns functional StdRegistryBridge
#[test]
fn test_default_bridge_returns_std_registry_bridge_happy() {
    let src = make_svc_registry();
    src.register(Arc::new(EchoSvc));
    let dst = make_handler_registry();

    let bridge: StdRegistryBridge = AlternativeBridge::default_bridge();
    let count = bridge.bridge(&src, &dst);

    assert_eq!(count, 1);
    assert_eq!(dst.get("echo").unwrap().id(), "echo");
}

/// @covers: RegistryBridge::default_bridge — returned bridge handles empty source
#[test]
fn test_default_bridge_on_empty_source_returns_zero_error() {
    let src = make_svc_registry();
    let dst = make_handler_registry();

    let bridge = StdRegistryBridge::default_bridge();
    let count = bridge.bridge(&src, &dst);

    assert_eq!(count, 0);
    assert!(dst.is_empty());
}

/// @covers: RegistryBridge::default_bridge — Copy: same instance works for multiple bridges
#[test]
fn test_default_bridge_is_copy_reusable_edge() {
    let src1 = make_svc_registry();
    src1.register(Arc::new(EchoSvc));
    let src2 = make_svc_registry();
    src2.register(Arc::new(GreetSvc));

    let bridge = StdRegistryBridge::default_bridge();
    let copy = bridge;

    let dst1 = make_handler_registry();
    let dst2 = make_handler_registry();
    let c1 = bridge.bridge(&src1, &dst1);
    let c2 = copy.bridge(&src2, &dst2);

    assert_eq!(c1, 1);
    assert_eq!(c2, 1);
    assert_eq!(dst1.get("echo").unwrap().id(), "echo");
    assert_eq!(dst2.get("greet").unwrap().id(), "greet");
}
