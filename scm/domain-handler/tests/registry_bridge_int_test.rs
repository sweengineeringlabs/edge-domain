//! Integration tests — `RegistryBridge` trait via SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_handler::{
    BridgeRequest, BridgeResponse, EmptinessRequest, HandlerError, HandlerLookupRequest,
    HandlerRegistry, IdRequest, InProcessHandlerRegistry, LenRequest, ListIdsRequest,
    RegisterHandlerRequest, RegistryBridge, StdRegistryBridge,
};
use edge_application_service::{
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
        Box::pin(async move { Ok(format!("hello {req}")) })
    }
}

struct AlternativeBridge;

impl RegistryBridge for AlternativeBridge {
    fn bridge<Req, Resp>(
        &self,
        _req: BridgeRequest<'_, Req, Resp>,
    ) -> Result<BridgeResponse, HandlerError>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        Ok(BridgeResponse { transferred: 0 })
    }
}

fn make_svc_registry() -> ServiceRegistryStore<String, String> {
    ServiceRegistryStore::default()
}

fn make_handler_registry() -> InProcessHandlerRegistry<String, String> {
    InProcessHandlerRegistry::default()
}

fn get_id(reg: &InProcessHandlerRegistry<String, String>, id: &str) -> Option<String> {
    reg.get(HandlerLookupRequest { id: id.to_string() })
        .unwrap()
        .handler
        .map(|h| h.id(IdRequest).unwrap().id)
}

/// @covers: RegistryBridge::bridge — all services transferred to handler registry
#[test]
fn test_bridge_transfers_all_services_happy() {
    let src = make_svc_registry();
    src.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    src.register(&RegisterServiceRequest::new(Arc::new(GreetSvc)))
        .unwrap();
    let dst = make_handler_registry();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 2);
    assert_eq!(dst.len(LenRequest).unwrap().count, 2);
    assert_eq!(get_id(&dst, "echo"), Some("echo".to_string()));
    assert_eq!(get_id(&dst, "greet"), Some("greet".to_string()));
}

/// @covers: RegistryBridge::bridge — empty source registry returns 0
#[test]
fn test_bridge_empty_registry_returns_zero_error() {
    let src = make_svc_registry();
    let dst = make_handler_registry();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 0);
    assert_eq!(dst.len(LenRequest).unwrap().count, 0);
    assert!(dst.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

/// @covers: RegistryBridge::bridge — second bridge into non-empty registry accumulates
#[test]
fn test_bridge_into_non_empty_registry_accumulates_edge() {
    let src = make_svc_registry();
    src.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    let dst = make_handler_registry();
    dst.register(RegisterHandlerRequest::new(Arc::new(
        edge_application_handler::EchoHandler::<String>::from(("pre", "/pre")),
    )))
    .unwrap();

    let result = StdRegistryBridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 1);
    assert_eq!(dst.len(LenRequest).unwrap().count, 2);
    assert!(get_id(&dst, "echo").is_some());
    assert!(get_id(&dst, "pre").is_some());
}

/// @covers: RegistryBridge::default_bridge — returns functional StdRegistryBridge
#[test]
fn test_default_bridge_returns_std_registry_bridge_happy() {
    let src = make_svc_registry();
    src.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    let dst = make_handler_registry();

    let bridge: StdRegistryBridge = AlternativeBridge::default_bridge();
    let result = bridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 1);
    assert_eq!(get_id(&dst, "echo"), Some("echo".to_string()));
}

/// @covers: RegistryBridge::default_bridge — returned bridge handles empty source
#[test]
fn test_default_bridge_on_empty_source_returns_zero_error() {
    let src = make_svc_registry();
    let dst = make_handler_registry();

    let bridge = StdRegistryBridge::default_bridge();
    let result = bridge.bridge(BridgeRequest {
        src: &src,
        dst: &dst,
    });

    assert_eq!(result.unwrap().transferred, 0);
    assert!(dst.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: RegistryBridge::default_bridge — Copy: same instance works for multiple bridges
#[test]
fn test_default_bridge_is_copy_reusable_edge() {
    let src1 = make_svc_registry();
    src1.register(&RegisterServiceRequest::new(Arc::new(EchoSvc)))
        .unwrap();
    let src2 = make_svc_registry();
    src2.register(&RegisterServiceRequest::new(Arc::new(GreetSvc)))
        .unwrap();

    let bridge = StdRegistryBridge::default_bridge();
    let copy = bridge;

    let dst1 = make_handler_registry();
    let dst2 = make_handler_registry();
    let c1 = bridge.bridge(BridgeRequest {
        src: &src1,
        dst: &dst1,
    });
    let c2 = copy.bridge(BridgeRequest {
        src: &src2,
        dst: &dst2,
    });

    assert_eq!(c1.unwrap().transferred, 1);
    assert_eq!(c2.unwrap().transferred, 1);
    assert_eq!(get_id(&dst1, "echo"), Some("echo".to_string()));
    assert_eq!(get_id(&dst2, "greet"), Some("greet".to_string()));
}
