//! Integration tests for `ServiceRegistryStore` — the in-process service registry implementation.

use std::sync::Arc;

use edge_domain_service::{
    Service, ServiceError, ServiceRegistry, ServiceRegistryStore, NameRequest, NameResponse,
    RegisterServiceRequest, ServiceLookupRequest, LenRequest, EmptinessRequest,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Constant(String, i32);
impl Service for Constant {
    type Request = i32;
    type Response = i32;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: self.0.clone(),
        })
    }
    fn execute(&self, _req: i32) -> BoxFuture<'_, Result<i32, ServiceError>> {
        let val = self.1;
        Box::pin(async move { Ok(val) })
    }
}

/// @covers: ServiceRegistryStore::default
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<i32, i32> = ServiceRegistryStore::default();
    let result = reg.len(LenRequest);
    assert_eq!(result.unwrap().count, 0);
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: ServiceRegistryStore::default
#[test]
fn test_default_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<i32, i32> = ServiceRegistryStore::default();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: ServiceRegistry::register and get
#[test]
fn test_register_then_get_retrieves_service_edge() {
    let reg: ServiceRegistryStore<i32, i32> = ServiceRegistryStore::default();
    let svc = Arc::new(Constant("forty-two".into(), 42));
    let req = RegisterServiceRequest {
        service: svc.clone(),
    };
    let _ = reg.register(req);
    let lookup_req = ServiceLookupRequest {
        name: "forty-two".to_string(),
    };
    let result = reg.get(lookup_req);
    assert!(result.unwrap().service.is_some());
}
