//! Comprehensive tests for ServiceRegistry trait methods.

use edge_domain_service::*;
use std::sync::Arc;

/// Helper to create a test service registry
fn make_registry() -> ServiceRegistryStore<(), ()> {
    ServiceRegistryStore::default()
}

// ===== register method =====
/// @covers: ServiceRegistry::register
#[test]
fn test_service_registry_register_stores_service_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let result = reg.register(&req);
    assert_eq!(result, Ok(RegisterServiceResponse));
}

/// @covers: ServiceRegistry::register
#[test]
fn test_service_registry_register_multiple_happy() {
    let reg = make_registry();
    for _ in 0..3 {
        let req = RegisterServiceRequest::new(Arc::new(NoopService));
        assert_eq!(reg.register(&req), Ok(RegisterServiceResponse));
    }
}

/// @covers: ServiceRegistry::register
#[test]
fn test_service_registry_register_replaces_existing_edge() {
    let reg = make_registry();
    let req1 = RegisterServiceRequest::new(Arc::new(NoopService));
    let req2 = RegisterServiceRequest::new(Arc::new(NoopService));
    assert_eq!(reg.register(&req1), Ok(RegisterServiceResponse));
    assert_eq!(reg.register(&req2), Ok(RegisterServiceResponse));
    let len_result = reg.len(LenRequest);
    assert_eq!(len_result.unwrap().count, 1);
}

// ===== deregister method =====
/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_deregister_present_returns_ok_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let deregister_req = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let result = reg.deregister(&deregister_req);
    assert!(result.is_ok());
    assert!(result.unwrap().was_present);
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_deregister_missing_returns_false_error() {
    let reg = make_registry();
    let deregister_req = ServiceRemovalRequest {
        name: "missing".to_string(),
    };
    let result = reg.deregister(&deregister_req);
    assert!(result.is_ok());
    assert!(!result.unwrap().was_present);
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_deregister_idempotent_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let deregister_req1 = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let deregister_req2 = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let result1 = reg.deregister(&deregister_req1);
    let result2 = reg.deregister(&deregister_req2);
    assert!(result1.unwrap().was_present);
    assert!(!result2.unwrap().was_present);
}

// ===== get method =====
/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_get_present_returns_some_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let lookup_req = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_ok());
    assert!(result.unwrap().service.is_some());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_get_missing_returns_none_error() {
    let reg = make_registry();
    let lookup_req = ServiceLookupRequest {
        name: "missing".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_ok());
    assert!(result.unwrap().service.is_none());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_get_returns_same_service_edge() {
    let reg = make_registry();
    let service = Arc::new(NoopService);
    let req = RegisterServiceRequest::new(service.clone());
    let _ = reg.register(&req);
    let lookup_req = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_ok());
    let retrieved = result.unwrap().service;
    assert!(retrieved.is_some());
}

// ===== list_names method =====
/// @covers: ServiceRegistry::list_names
#[test]
fn test_service_registry_list_names_empty_registry_happy() {
    let reg = make_registry();
    let req = ListNamesRequest;
    let result = reg.list_names(req);
    assert!(result.is_ok());
    assert!(result.unwrap().names.is_empty());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_service_registry_list_names_populated_registry_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let list_req = ListNamesRequest;
    let result = reg.list_names(list_req);
    assert!(result.is_ok());
    assert!(!result.unwrap().names.is_empty());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_service_registry_list_names_includes_registered_error() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let list_req = ListNamesRequest;
    let result = reg.list_names(list_req);
    assert!(result.is_ok());
    let names = result.unwrap().names;
    assert!(names.contains(&"noop".to_string()));
}

// ===== len method =====
/// @covers: ServiceRegistry::len
#[test]
fn test_service_registry_len_empty_registry_happy() {
    let reg = make_registry();
    let req = LenRequest;
    let result = reg.len(req);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().count, 0);
}

/// @covers: ServiceRegistry::len
#[test]
fn test_service_registry_len_after_register_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let len_req = LenRequest;
    let result = reg.len(len_req);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().count, 1);
}

/// @covers: ServiceRegistry::len
#[test]
fn test_service_registry_len_correct_after_operations_error() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let deregister_req = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let _ = reg.deregister(&deregister_req);
    let len_req = LenRequest;
    let result = reg.len(len_req);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().count, 0);
}

// ===== is_empty method =====
/// @covers: ServiceRegistry::is_empty
#[test]
fn test_service_registry_is_empty_fresh_registry_happy() {
    let reg = make_registry();
    let req = EmptinessRequest;
    let result = reg.is_empty(req);
    assert!(result.is_ok());
    assert!(result.unwrap().empty);
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_service_registry_is_empty_after_register_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let emptiness_req = EmptinessRequest;
    let result = reg.is_empty(emptiness_req);
    assert!(result.is_ok());
    assert!(!result.unwrap().empty);
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_service_registry_is_empty_after_deregister_error() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let deregister_req = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let _ = reg.deregister(&deregister_req);
    let emptiness_req = EmptinessRequest;
    let result = reg.is_empty(emptiness_req);
    assert!(result.is_ok());
    assert!(result.unwrap().empty);
}
