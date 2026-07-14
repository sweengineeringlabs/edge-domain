//! Tests for [`RegisterServiceRequest`].

use edge_application_service::{
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
    if let Err(err) = reg.register(&req) {
        panic!("expected Ok, got Err: {err:?}");
    }

    let lookup = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let found = match reg.get(&lookup) {
        Ok(response) => response.service,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    };
    match found {
        Some(found_svc) => assert!(Arc::ptr_eq(&found_svc, &svc)),
        None => panic!("expected the registered service to be found by name"),
    }
}
