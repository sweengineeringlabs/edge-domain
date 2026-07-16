//! Tests for [`ServiceRegistryStore`] — the in-process service registry implementation.

use edge_application_service::{EmptinessRequest, NoopRequest, NoopResponse, ServiceRegistry, ServiceRegistryStore};

#[allow(dead_code)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_default_creates_empty_registry_happy() {
    let reg: ServiceRegistryStore<NoopRequest, NoopResponse> = ServiceRegistryStore::default();
    match reg.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistryStore
#[test]
fn test_service_registry_store_implements_service_registry_edge() {
    let reg: ServiceRegistryStore<TextPayload, TextPayload> = ServiceRegistryStore::default();
    let req = EmptinessRequest;
    match reg.is_empty(req) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}
