//! Tests for [`ServiceRegistryStore`] — the in-process service registry implementation.

use edge_domain_service::{EmptinessRequest, ServiceRegistry, ServiceRegistryStore};

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_default_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
    match reg.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_implements_service_registry_edge() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    let req = EmptinessRequest;
    match reg.is_empty(req) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}
