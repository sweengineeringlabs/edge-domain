//! Tests for [`RegisterServiceRequest`].

use edge_domain_service::{
    NoopService, RegisterServiceRequest, Service, ServiceLookupRequest, ServiceRegistry,
    ServiceRegistryStore,
};
use std::sync::Arc;

/// @covers: RegisterServiceRequest — constructible with service
#[test]
fn test_register_service_request_constructible_happy() {
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let _req = RegisterServiceRequest::new(svc);
}

/// @covers: RegisterServiceRequest — holds service reference used during registration
#[test]
fn test_register_service_request_holds_service_edge() {
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let req = RegisterServiceRequest::new(Arc::clone(&svc));

    let reg: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
    reg.register(&req).unwrap();

    let lookup = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let found = reg.get(&lookup).unwrap().service;
    assert!(found.is_some());
}
