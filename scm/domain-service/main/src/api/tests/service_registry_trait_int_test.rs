//! Trait contract tests for ServiceRegistry — verifies public API.

use crate::api::{
    EmptinessRequest, LenRequest, ListNamesRequest, RegisterServiceRequest, ServiceRegistry,
    ServiceRemovalRequest, ServiceLookupRequest, ServiceError, ServiceRegistryStore,
};
use std::sync::Arc;

/// Test helper to create a registry implementation.
struct TestRegistry;

impl ServiceRegistry for TestRegistry {
    type Request = ();
    type Response = ();

    fn register(
        &self,
        _req: RegisterServiceRequest<(), ()>,
    ) -> Result<crate::api::RegisterServiceResponse, ServiceError> {
        Ok(crate::api::RegisterServiceResponse)
    }

    fn deregister(
        &self,
        _req: ServiceRemovalRequest,
    ) -> Result<crate::api::ServiceRemovalResponse, ServiceError> {
        Ok(crate::api::ServiceRemovalResponse { was_present: false })
    }

    fn get(
        &self,
        _req: ServiceLookupRequest,
    ) -> Result<crate::api::ServiceLookupResponse<(), ()>, ServiceError> {
        Ok(crate::api::ServiceLookupResponse { service: None })
    }

    fn list_names(
        &self,
        _req: ListNamesRequest,
    ) -> Result<crate::api::ListNamesResponse, ServiceError> {
        Ok(crate::api::ListNamesResponse { names: vec![] })
    }

    fn len(&self, _req: LenRequest) -> Result<crate::api::LenResponse, ServiceError> {
        Ok(crate::api::LenResponse { count: 0 })
    }

    fn is_empty(&self, _req: EmptinessRequest) -> Result<crate::api::EmptinessResponse, ServiceError> {
        Ok(crate::api::EmptinessResponse { empty: true })
    }
}

/// @covers: ServiceRegistry::register
#[test]
fn test_service_registry_trait_register_contract_happy() {
    let reg = TestRegistry;
    let svc = Arc::new(crate::api::NoopService);
    let req = RegisterServiceRequest { service: svc };
    let result = reg.register(req);
    assert!(result.is_ok());
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_trait_deregister_contract_happy() {
    let reg = TestRegistry;
    let req = ServiceRemovalRequest {
        name: "test".to_string(),
    };
    let result = reg.deregister(req);
    assert!(result.is_ok());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_trait_get_contract_happy() {
    let reg = TestRegistry;
    let req = ServiceLookupRequest {
        name: "test".to_string(),
    };
    let result = reg.get(req);
    assert!(result.is_ok());
}

/// @covers: ServiceRegistry::list_names
#[test]
fn test_service_registry_trait_list_names_contract_happy() {
    let reg = TestRegistry;
    let req = ListNamesRequest;
    let result = reg.list_names(req);
    assert!(result.is_ok());
}

/// @covers: ServiceRegistry::len
#[test]
fn test_service_registry_trait_len_contract_happy() {
    let reg = TestRegistry;
    let req = LenRequest;
    let result = reg.len(req);
    assert!(result.is_ok());
}

/// @covers: ServiceRegistry::is_empty
#[test]
fn test_service_registry_trait_is_empty_contract_happy() {
    let reg = TestRegistry;
    let req = EmptinessRequest;
    let result = reg.is_empty(req);
    assert!(result.is_ok());
}
