//! Integration tests for `Service` and `ServiceRegistry`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, Service, ServiceError, ServiceRegistry};
use edge_domain_service::{
    EmptinessRequest, LenRequest, NameRequest, NameResponse, RegisterServiceRequest,
    ServiceLookupRequest, ServiceRegistryStore, ServiceRemovalRequest,
};
use futures::future::BoxFuture;
use std::sync::Arc;

struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct FailingService;

impl Service for FailingService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "failing".to_string(),
        })
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async { Err(ServiceError::RuleViolation("always fails".into())) })
    }
}

/// @covers: ServiceRegistry::register, ServiceRegistry::get
#[test]
fn test_service_registry_struct_register_and_get_retrieves_service() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    reg.register(&RegisterServiceRequest::new(Arc::new(EchoService)))
        .unwrap();
    assert!(reg
        .get(&ServiceLookupRequest {
            name: "echo".to_string()
        })
        .unwrap()
        .service
        .is_some());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_struct_get_returns_none_for_missing_name() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    assert!(reg
        .get(&ServiceLookupRequest {
            name: "missing".to_string()
        })
        .unwrap()
        .service
        .is_none());
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_struct_deregister_removes_service() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    reg.register(&RegisterServiceRequest::new(Arc::new(EchoService)))
        .unwrap();
    assert!(
        reg.deregister(&ServiceRemovalRequest {
            name: "echo".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(reg
        .get(&ServiceLookupRequest {
            name: "echo".to_string()
        })
        .unwrap()
        .service
        .is_none());
}

/// @covers: ServiceRegistry::len, ServiceRegistry::is_empty
#[test]
fn test_service_registry_struct_len_reflects_registration_count() {
    let reg: ServiceRegistryStore<String, String> = ServiceRegistryStore::default();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    reg.register(&RegisterServiceRequest::new(Arc::new(EchoService)))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

/// @covers: Service::execute
#[tokio::test]
async fn test_service_trait_execute_returns_result() {
    let svc = EchoService;
    let result = svc.execute("ping".into()).await.unwrap();
    assert_eq!(result, "ping");
}

/// @covers: Service::execute
#[tokio::test]
async fn test_service_trait_execute_propagates_error() {
    let svc = FailingService;
    assert!(svc.execute("x".into()).await.is_err());
}

/// @covers: new_service_registry
#[test]
fn test_factory_fn_new_service_registry_returns_empty_arc_registry() {
    let reg = Domain.new_service_registry::<String, String>();
    assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
}
