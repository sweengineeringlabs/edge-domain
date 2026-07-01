//! Tests for [`RegisterServiceRequest`].

use edge_domain_service::RegisterServiceRequest;
use std::sync::Arc;

/// @covers: RegisterServiceRequest — constructible with service
#[test]
fn test_register_service_request_constructible_happy() {
    use edge_domain_service::{NoopService, Service};
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let req = RegisterServiceRequest {
        service: svc.clone(),
    };
    assert_eq!(req.service.as_ref() as *const _, svc.as_ref() as *const _);
}

/// @covers: RegisterServiceRequest — holds service reference
#[test]
fn test_register_service_request_holds_service_edge() {
    use edge_domain_service::{NoopService, Service};
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let req = RegisterServiceRequest {
        service: svc.clone(),
    };
    assert!(Arc::ptr_eq(&req.service, &svc));
}
