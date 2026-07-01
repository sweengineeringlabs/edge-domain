//! Tests for [`ServiceRegistryStore`] — the in-process service registry implementation.

use edge_domain_service::{ServiceRegistry, ServiceRegistryStore, EmptinessRequest};

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_default_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_implements_service_registry_edge() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    let req = EmptinessRequest;
    assert!(reg.is_empty(req).unwrap().empty);
}
