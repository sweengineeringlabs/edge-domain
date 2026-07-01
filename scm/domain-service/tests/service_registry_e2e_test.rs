//! End-to-end contract tests for the `ServiceRegistry` trait, exercised
//! through test-double implementations via the crate's public API.

use edge_domain_service::{
    EmptinessRequest, LenRequest, ListNamesRequest, NoopService, RegisterServiceRequest,
    RegisterServiceResponse, Service, ServiceError, ServiceLookupRequest, ServiceRegistry,
    ServiceRegistryStore, ServiceRemovalRequest, StdServiceRegistryFactory,
};
use std::sync::Arc;

struct TestRegistry;

impl ServiceRegistry for TestRegistry {
    type Request = ();
    type Response = ();

    fn register(
        &self,
        _req: &RegisterServiceRequest<(), ()>,
    ) -> Result<RegisterServiceResponse, ServiceError> {
        Ok(RegisterServiceResponse)
    }

    fn deregister(
        &self,
        _req: &ServiceRemovalRequest,
    ) -> Result<edge_domain_service::ServiceRemovalResponse, ServiceError> {
        Ok(edge_domain_service::ServiceRemovalResponse { was_present: false })
    }

    fn get(
        &self,
        _req: &ServiceLookupRequest,
    ) -> Result<edge_domain_service::ServiceLookupResponse<(), ()>, ServiceError> {
        Ok(edge_domain_service::ServiceLookupResponse { service: None })
    }

    fn list_names(
        &self,
        _req: ListNamesRequest,
    ) -> Result<edge_domain_service::ListNamesResponse, ServiceError> {
        Ok(edge_domain_service::ListNamesResponse { names: vec![] })
    }

    fn len(&self, _req: LenRequest) -> Result<edge_domain_service::LenResponse, ServiceError> {
        Ok(edge_domain_service::LenResponse { count: 0 })
    }

    fn is_empty(
        &self,
        _req: EmptinessRequest,
    ) -> Result<edge_domain_service::EmptinessResponse, ServiceError> {
        Ok(edge_domain_service::EmptinessResponse { empty: true })
    }

    fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }

    fn noop_service() -> NoopService {
        NoopService
    }

    fn new_store() -> ServiceRegistryStore<(), ()> {
        ServiceRegistryStore::default()
    }
}

struct FailingRegistry;

impl ServiceRegistry for FailingRegistry {
    type Request = ();
    type Response = ();

    fn register(
        &self,
        _req: &RegisterServiceRequest<(), ()>,
    ) -> Result<RegisterServiceResponse, ServiceError> {
        Err(ServiceError::RuleViolation(
            "registration rejected".to_string(),
        ))
    }

    fn deregister(
        &self,
        _req: &ServiceRemovalRequest,
    ) -> Result<edge_domain_service::ServiceRemovalResponse, ServiceError> {
        Err(ServiceError::NotFound("registry unavailable".to_string()))
    }

    fn get(
        &self,
        _req: &ServiceLookupRequest,
    ) -> Result<edge_domain_service::ServiceLookupResponse<(), ()>, ServiceError> {
        Err(ServiceError::NotFound("registry unavailable".to_string()))
    }

    fn list_names(
        &self,
        _req: ListNamesRequest,
    ) -> Result<edge_domain_service::ListNamesResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn len(&self, _req: LenRequest) -> Result<edge_domain_service::LenResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn is_empty(
        &self,
        _req: EmptinessRequest,
    ) -> Result<edge_domain_service::EmptinessResponse, ServiceError> {
        Err(ServiceError::Unavailable("registry offline".to_string()))
    }

    fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }

    fn noop_service() -> NoopService {
        NoopService
    }

    fn new_store() -> ServiceRegistryStore<(), ()> {
        ServiceRegistryStore::default()
    }
}

/// @covers: ServiceRegistry::register
#[test]
fn test_register_stores_service_happy() {
    let reg = TestRegistry;
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    assert_eq!(reg.register(&req), Ok(RegisterServiceResponse));
}

/// @covers: ServiceRegistry::register
#[test]
fn test_register_rejected_error() {
    let reg = FailingRegistry;
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    assert!(reg.register(&req).is_err());
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_deregister_missing_happy() {
    let reg = TestRegistry;
    let req = ServiceRemovalRequest {
        name: "x".to_string(),
    };
    match reg.deregister(&req) {
        Ok(response) => assert!(!response.was_present),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_deregister_unavailable_error() {
    let reg = FailingRegistry;
    let req = ServiceRemovalRequest {
        name: "x".to_string(),
    };
    assert!(reg.deregister(&req).is_err());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_get_returns_option_happy() {
    let reg = TestRegistry;
    let req = ServiceLookupRequest {
        name: "x".to_string(),
    };
    match reg.get(&req) {
        Ok(response) => assert!(response.service.is_none()),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::get
#[test]
fn test_get_unavailable_error() {
    let reg = FailingRegistry;
    let req = ServiceLookupRequest {
        name: "x".to_string(),
    };
    assert!(reg.get(&req).is_err());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_list_names_returns_vec_happy() {
    let reg = TestRegistry;
    match reg.list_names(ListNamesRequest) {
        Ok(response) => assert!(response.names.is_empty()),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_list_names_unavailable_error() {
    let reg = FailingRegistry;
    assert!(reg.list_names(ListNamesRequest).is_err());
}

/// @covers: ServiceRegistry::len
#[test]
fn test_len_returns_count_happy() {
    let reg = TestRegistry;
    match reg.len(LenRequest) {
        Ok(response) => assert_eq!(response.count, 0),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::len
#[test]
fn test_len_unavailable_error() {
    let reg = FailingRegistry;
    assert!(reg.len(LenRequest).is_err());
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_is_empty_returns_bool_happy() {
    let reg = TestRegistry;
    match reg.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_is_empty_unavailable_error() {
    let reg = FailingRegistry;
    assert!(reg.is_empty(EmptinessRequest).is_err());
}

/// @covers: ServiceRegistry::default_factory
#[test]
fn test_default_factory_returns_factory_happy() {
    let factory = TestRegistry::default_factory();
    assert_eq!(factory, StdServiceRegistryFactory);
}

/// @covers: ServiceRegistry::default_factory
#[test]
fn test_default_factory_consistent_edge() {
    let f1 = TestRegistry::default_factory();
    let f2 = FailingRegistry::default_factory();
    assert_eq!(f1, f2);
}

/// @covers: ServiceRegistry::noop_service
#[test]
fn test_noop_service_returns_noop_happy() {
    let noop = TestRegistry::noop_service();
    match noop.name(edge_domain_service::NameRequest) {
        Ok(response) => assert_eq!(response.name, "noop"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::noop_service
#[test]
fn test_noop_service_consistent_edge() {
    let noop1 = TestRegistry::noop_service();
    let noop2 = FailingRegistry::noop_service();
    let n1 = noop1.name(edge_domain_service::NameRequest);
    let n2 = noop2.name(edge_domain_service::NameRequest);
    assert_eq!(n1, n2);
}

/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_returns_empty_registry_happy() {
    let store = TestRegistry::new_store();
    match store.is_empty(EmptinessRequest) {
        Ok(response) => assert!(response.empty),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_usable_for_registration_error() {
    let store = TestRegistry::new_store();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    assert_eq!(store.register(&req), Ok(RegisterServiceResponse));
    match store.len(LenRequest) {
        Ok(response) => assert_eq!(response.count, 1),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: ServiceRegistry::new_store
#[test]
fn test_new_store_instances_are_independent_edge() {
    let store1 = TestRegistry::new_store();
    let store2 = TestRegistry::new_store();
    let req = RegisterServiceRequest::new(Arc::new(NoopService));
    if let Err(err) = store1.register(&req) {
        panic!("expected Ok, got Err: {err:?}");
    }
    let len1 = store1.len(LenRequest);
    let len2 = store2.len(LenRequest);
    match (len1, len2) {
        (Ok(r1), Ok(r2)) => {
            assert_eq!(r1.count, 1);
            assert_eq!(r2.count, 0);
        }
        (r1, r2) => panic!("expected both Ok, got {r1:?} and {r2:?}"),
    }
}
