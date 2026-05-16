//! Integration tests for `Service` and `ServiceRegistry`.

use async_trait::async_trait;
use edge_domain::{new_service_registry, Service, ServiceError, ServiceRegistry};
use std::sync::Arc;

struct EchoService;

#[async_trait]
impl Service<String, String> for EchoService {
    fn name(&self) -> &str {
        "echo"
    }
    async fn execute(&self, req: String) -> Result<String, ServiceError> {
        Ok(req)
    }
}

struct FailingService;

#[async_trait]
impl Service<String, String> for FailingService {
    fn name(&self) -> &str {
        "failing"
    }
    async fn execute(&self, _req: String) -> Result<String, ServiceError> {
        Err(ServiceError::RuleViolation("always fails".into()))
    }
}

/// @covers: ServiceRegistry::register, ServiceRegistry::get
#[test]
fn test_service_registry_struct_register_and_get_retrieves_service() {
    let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
    reg.register(Arc::new(EchoService));
    assert!(reg.get("echo").is_some());
}

/// @covers: ServiceRegistry::get
#[test]
fn test_service_registry_struct_get_returns_none_for_missing_name() {
    let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
    assert!(reg.get("missing").is_none());
}

/// @covers: ServiceRegistry::deregister
#[test]
fn test_service_registry_struct_deregister_removes_service() {
    let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
    reg.register(Arc::new(EchoService));
    assert!(reg.deregister("echo"));
    assert!(reg.get("echo").is_none());
}

/// @covers: ServiceRegistry::len, ServiceRegistry::is_empty
#[test]
fn test_service_registry_struct_len_reflects_registration_count() {
    let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
    assert!(reg.is_empty());
    reg.register(Arc::new(EchoService));
    assert_eq!(reg.len(), 1);
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
    let reg: Arc<ServiceRegistry<String, String>> = new_service_registry();
    assert!(reg.is_empty());
}
