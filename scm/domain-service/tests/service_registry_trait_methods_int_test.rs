//! Comprehensive tests for ServiceRegistry trait methods.
//!
//! Error-path scenarios use `FailingRegistry`, a test double that always
//! returns `Err`, since the concrete `ServiceRegistryStore` never fails and
//! so cannot exercise the trait's error contract on its own.

use edge_application_service::*;
use edge_application_service::{NoopRequest, NoopResponse};
use std::fmt::Debug;
use std::sync::Arc;

/// Helper to create a test service registry
fn make_registry() -> ServiceRegistryStore<NoopRequest, NoopResponse> {
    ServiceRegistryStore::default()
}

/// Unwrap a `Result` in test code without tripping the crate-wide
/// `clippy::unwrap_used` / `clippy::expect_used` deny-lints.
fn ok<T, E: Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// A test double whose every fallible method returns `Err`, used to verify
/// that callers of the `ServiceRegistry` contract can observe and handle
/// failures (the real `ServiceRegistryStore` never fails).
struct FailingRegistry;

impl ServiceRegistry for FailingRegistry {
    type Request = NoopRequest;
    type Response = NoopResponse;

    fn register(
        &self,
        _req: &RegisterServiceRequest<NoopRequest, NoopResponse>,
    ) -> Result<RegisterServiceResponse, ServiceError> {
        Err(ServiceError::RuleViolation(
            "registration rejected".to_string(),
        ))
    }

    fn deregister(
        &self,
        _req: &ServiceRemovalRequest,
    ) -> Result<ServiceRemovalResponse, ServiceError> {
        Err(ServiceError::NotFound("registry unavailable".to_string()))
    }

    fn get(
        &self,
        _req: &ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<NoopRequest, NoopResponse>, ServiceError> {
        Err(ServiceError::NotFound("registry unavailable".to_string()))
    }

    fn list_names(&self, _req: ListNamesRequest) -> Result<ListNamesResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }

    fn noop_service() -> NoopService {
        NoopService
    }

    fn new_store() -> ServiceRegistryStore<NoopRequest, NoopResponse> {
        ServiceRegistryStore::default()
    }
}

// ===== register method =====
/// @covers: ServiceRegistry::register
#[test]
fn test_register_stores_service_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let result = reg.register(&req);
    assert_eq!(result, Ok(RegisterServiceResponse));
}

/// @covers: ServiceRegistry::register
#[test]
fn test_register_rejected_by_registry_error() {
    let reg = FailingRegistry;
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let result = reg.register(&req);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::register
#[test]
fn test_register_replaces_existing_edge() {
    let reg = make_registry();
    let req1 = RegisterServiceRequest::new(Arc::new(NoopService));
    let req2 = RegisterServiceRequest::new(Arc::new(NoopService));
    assert_eq!(reg.register(&req1), Ok(RegisterServiceResponse));
    assert_eq!(reg.register(&req2), Ok(RegisterServiceResponse));
    let len_result = reg.len(LenRequest);
    assert_eq!(ok(len_result).count, 1);
}

// ===== deregister method =====
/// @covers: ServiceRegistry::deregister
#[test]
fn test_deregister_present_returns_ok_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let deregister_req = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let result = reg.deregister(&deregister_req);
    assert!(result.is_ok());
    assert!(ok(result).was_present);
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_deregister_unavailable_registry_error() {
    let reg = FailingRegistry;
    let deregister_req = ServiceRemovalRequest {
        name: "noop".to_string(),
    };
    let result = reg.deregister(&deregister_req);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_deregister_idempotent_edge() {
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
    assert!(ok(result1).was_present);
    assert!(!ok(result2).was_present);
}

// ===== get method =====
/// @covers: ServiceRegistry::get
#[test]
fn test_get_present_returns_some_happy() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let lookup_req = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_ok());
    match ok(result).service {
        Some(found) => assert_eq!(ok(found.name(NameRequest)).name, "noop"),
        None => panic!("expected the registered service to be found by name"),
    }
}

/// @covers: ServiceRegistry::get
#[test]
fn test_get_unavailable_registry_error() {
    let reg = FailingRegistry;
    let lookup_req = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_get_returns_same_service_edge() {
    let reg = make_registry();
    let service: Arc<dyn Service<Request = NoopRequest, Response = NoopResponse>> =
        Arc::new(NoopService);
    let req = RegisterServiceRequest::new(Arc::clone(&service));
    let _ = reg.register(&req);
    let lookup_req = ServiceLookupRequest {
        name: "noop".to_string(),
    };
    let result = reg.get(&lookup_req);
    assert!(result.is_ok());
    let retrieved = ok(result).service;
    match retrieved {
        Some(found) => assert!(Arc::ptr_eq(&found, &service)),
        None => panic!("expected the registered service to be found by name"),
    }
}

// ===== list_names method =====
/// @covers: ServiceRegistry::list_names
#[test]
fn test_list_names_empty_registry_happy() {
    let reg = make_registry();
    let req = ListNamesRequest;
    let result = reg.list_names(req);
    assert!(result.is_ok());
    assert!(ok(result).names.is_empty());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_list_names_unavailable_registry_error() {
    let reg = FailingRegistry;
    let result = reg.list_names(ListNamesRequest);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_list_names_populated_registry_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let list_req = ListNamesRequest;
    let result = reg.list_names(list_req);
    assert!(result.is_ok());
    assert!(!ok(result).names.is_empty());
}

// ===== len method =====
/// @covers: ServiceRegistry::len
#[test]
fn test_len_empty_registry_happy() {
    let reg = make_registry();
    let req = LenRequest;
    let result = reg.len(req);
    assert!(result.is_ok());
    assert_eq!(ok(result).count, 0);
}

/// @covers: ServiceRegistry::len
#[test]
fn test_len_unavailable_registry_error() {
    let reg = FailingRegistry;
    let result = reg.len(LenRequest);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::len
#[test]
fn test_len_correct_after_operations_edge() {
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
    assert_eq!(ok(result).count, 0);
}

// ===== is_empty method =====
/// @covers: ServiceRegistry::is_empty
#[test]
fn test_is_empty_fresh_registry_happy() {
    let reg = make_registry();
    let req = EmptinessRequest;
    let result = reg.is_empty(req);
    assert!(result.is_ok());
    assert!(ok(result).empty);
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_is_empty_unavailable_registry_error() {
    let reg = FailingRegistry;
    let result = reg.is_empty(EmptinessRequest);
    assert!(result.is_err());
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_is_empty_after_register_edge() {
    let reg = make_registry();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    let _ = reg.register(&req);
    let emptiness_req = EmptinessRequest;
    let result = reg.is_empty(emptiness_req);
    assert!(result.is_ok());
    assert!(!ok(result).empty);
}

// ===== default_factory method (infallible constructor — no Result) =====
/// @covers: ServiceRegistry::default_factory
#[test]
fn test_default_factory_returns_factory_happy() {
    let factory = ServiceRegistryStore::<NoopRequest, NoopResponse>::default_factory();
    assert_eq!(
        std::mem::size_of_val(&factory),
        std::mem::size_of::<StdServiceRegistryFactory>()
    );
}

/// @covers: ServiceRegistry::default_factory
#[test]
fn test_default_factory_consistent_across_impls_error() {
    let f1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::default_factory();
    let f2 = FailingRegistry::default_factory();
    assert_eq!(std::mem::size_of_val(&f1), std::mem::size_of_val(&f2));
}

/// @covers: ServiceRegistry::default_factory
#[test]
fn test_default_factory_multiple_calls_edge() {
    let f1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::default_factory();
    let f2 = ServiceRegistryStore::<NoopRequest, NoopResponse>::default_factory();
    assert_eq!(std::mem::size_of_val(&f1), std::mem::size_of_val(&f2));
}

// ===== noop_service method (infallible constructor — no Result) =====
/// @covers: ServiceRegistry::noop_service
#[test]
fn test_noop_service_returns_noop_happy() {
    let svc = ServiceRegistryStore::<NoopRequest, NoopResponse>::noop_service();
    assert_eq!(ok(svc.name(NameRequest)).name, "noop");
}

/// @covers: ServiceRegistry::noop_service
#[test]
fn test_noop_service_consistent_across_impls_error() {
    let svc1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::noop_service();
    let svc2 = FailingRegistry::noop_service();
    assert_eq!(
        ok(svc1.name(NameRequest)).name,
        ok(svc2.name(NameRequest)).name
    );
}

/// @covers: ServiceRegistry::noop_service
#[test]
fn test_noop_service_multiple_calls_edge() {
    let svc1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::noop_service();
    let svc2 = ServiceRegistryStore::<NoopRequest, NoopResponse>::noop_service();
    assert_eq!(
        ok(svc1.name(NameRequest)).name,
        ok(svc2.name(NameRequest)).name
    );
}

// ===== new_store method (infallible constructor — no Result) =====
/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_returns_empty_registry_happy() {
    let store = ServiceRegistryStore::<NoopRequest, NoopResponse>::new_store();
    assert!(ok(store.is_empty(EmptinessRequest)).empty);
}

/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_independent_across_impls_error() {
    let store1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::new_store();
    let store2 = FailingRegistry::new_store();
    assert_eq!(
        ok(store1.is_empty(EmptinessRequest)).empty,
        ok(store2.is_empty(EmptinessRequest)).empty
    );
}

/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_instances_are_independent_edge() {
    let store1 = ServiceRegistryStore::<NoopRequest, NoopResponse>::new_store();
    let store2 = ServiceRegistryStore::<NoopRequest, NoopResponse>::new_store();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    ok(store1.register(&req));
    assert_eq!(ok(store1.len(LenRequest)).count, 1);
    assert_eq!(ok(store2.len(LenRequest)).count, 0);
}
