//! Integration tests for `ServiceRegistry` — the in-process service registry type.

use std::sync::Arc;

use edge_domain_service::{Service, ServiceError, ServiceRegistry};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Constant(String, i32);
impl Service for Constant {
    type Request = i32;
    type Response = i32;

    fn name(&self) -> &str {
        &self.0
    }
    fn execute(&self, _req: i32) -> BoxFuture<'_, Result<i32, ServiceError>> {
        let val = self.1;
        Box::pin(async move { Ok(val) })
    }
}

/// @covers: ServiceRegistry::new — constructs an empty registry
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg: ServiceRegistry<i32, i32> = ServiceRegistry::new();
    assert_eq!(reg.len(), 0);
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistry::default — equivalent to new()
#[test]
fn test_default_is_equivalent_to_new_error() {
    let reg: ServiceRegistry<i32, i32> = ServiceRegistry::default();
    assert!(reg.is_empty());
}

/// @covers: ServiceRegistry — register then execute via get
#[test]
fn test_register_then_execute_produces_expected_value_edge() {
    let reg: ServiceRegistry<i32, i32> = ServiceRegistry::new();
    reg.register(Arc::new(Constant("forty-two".into(), 42)));
    let svc = reg.get("forty-two").expect("service must be present");
    let result = block_on(svc.execute(0));
    assert_eq!(result.ok(), Some(42));
}
