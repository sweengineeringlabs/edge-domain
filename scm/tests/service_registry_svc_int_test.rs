#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — ServiceRegistry is exported from the crate root.

use edge_domain::Domain;
use edge_domain::Service;
use edge_domain::ServiceError;
use std::sync::Arc;

struct Greeter;
impl Service<(), String> for Greeter {
    fn name(&self) -> &str {
        "greeter"
    }
    fn execute(&self, _: ()) -> futures::future::BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async { Ok("hello".into()) })
    }
}

#[test]
fn test_service_registry_svc_facade_register_and_get() {
    let reg = Domain::new_service_registry::<(), String>();
    reg.register(Arc::new(Greeter));
    assert!(reg.get("greeter").is_some());
}

#[test]
fn test_service_registry_svc_facade_missing_name_returns_none() {
    let reg = Domain::new_service_registry::<(), String>();
    assert!(reg.get("absent").is_none());
}
