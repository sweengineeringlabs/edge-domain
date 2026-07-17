//! Integration tests for `ServiceRegistryStore` — the in-process service registry implementation.

use std::sync::Arc;

use edge_application_service::{
    EmptinessRequest, LenRequest, NameRequest, NameResponse, RegisterServiceRequest, Service,
    ServiceError, ServiceLookupRequest, ServiceRegistry, ServiceRegistryStore,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

#[allow(dead_code)]
struct NumberPayload(i32);

impl edge_application_base::Request for NumberPayload {}
impl edge_application_base::Response for NumberPayload {}

struct Constant(String, i32);
impl Service for Constant {
    type Request = NumberPayload;
    type Response = NumberPayload;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: self.0.clone(),
        })
    }
    fn execute(&self, _req: NumberPayload) -> BoxFuture<'_, Result<NumberPayload, ServiceError>> {
        let val = self.1;
        Box::pin(async move { Ok(NumberPayload(val)) })
    }
}

/// @covers: ServiceRegistryStore::default
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<NumberPayload, NumberPayload> = ServiceRegistryStore::default();
    let result = reg.len(LenRequest);
    match result {
        Ok(response) => assert_eq!(response.count, 0),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
    match reg.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistryStore::default
#[test]
fn test_default_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<NumberPayload, NumberPayload> = ServiceRegistryStore::default();
    match reg.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::register and get
#[test]
fn test_register_then_get_retrieves_service_edge() {
    let reg: ServiceRegistryStore<NumberPayload, NumberPayload> = ServiceRegistryStore::default();
    let svc = Arc::new(Constant("forty-two".into(), 42));
    let req = RegisterServiceRequest::new(svc.clone());
    let _ = reg.register(&req);
    let lookup_req = ServiceLookupRequest {
        name: "forty-two".to_string(),
    };
    let result = reg.get(&lookup_req);
    let found = match result {
        Ok(response) => response.service,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    };
    match found {
        Some(found_svc) => {
            let name_result = found_svc.name(NameRequest);
            match name_result {
                Ok(name_resp) => assert_eq!(name_resp.name, "forty-two"),
                Err(err) => panic!("expected Ok, got Err: {err:?}"),
            }
        }
        None => panic!("expected the registered service to be found by name"),
    }
}
