//! Tests for [`ServiceLookupResponse`].

use edge_application_service::ServiceLookupResponse;
use std::sync::Arc;

/// @covers: ServiceLookupResponse — constructible with None
#[test]
fn test_service_lookup_response_none_happy() {
    let resp: ServiceLookupResponse<(), ()> = ServiceLookupResponse { service: None };
    assert!(resp.service.is_none());
}

/// @covers: ServiceLookupResponse — constructible with Some service
#[test]
fn test_service_lookup_response_some_happy() {
    use edge_application_service::{NoopService, Service};
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let resp = ServiceLookupResponse {
        service: Some(svc.clone()),
    };
    assert!(resp.service.is_some());
    match resp.service.as_ref() {
        Some(found) => assert!(Arc::ptr_eq(found, &svc)),
        None => panic!("expected Some, got None"),
    }
}

/// @covers: ServiceLookupResponse — differentiates None from Some
#[test]
fn test_service_lookup_response_none_vs_some_edge() {
    use edge_application_service::{NoopService, Service};
    let svc: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
    let resp_none: ServiceLookupResponse<(), ()> = ServiceLookupResponse { service: None };
    let resp_some = ServiceLookupResponse { service: Some(svc) };
    assert_ne!(resp_none.service.is_some(), resp_some.service.is_some());
}
