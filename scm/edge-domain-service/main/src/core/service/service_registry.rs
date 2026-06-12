//! `ServiceRegistry` trait impl for the [`ServiceRegistry`] struct — in-process registry.

use std::sync::Arc;

use crate::api::service::traits::service_registry::ServiceRegistry as ServiceRegistryTrait;
use crate::api::service::types::ServiceRegistry;
use crate::api::service::Service;

impl<Req, Resp> ServiceRegistryTrait<Req, Resp> for ServiceRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn register(&self, service: Arc<dyn Service<Req, Resp>>) {
        self.register(service);
    }

    fn deregister(&self, name: &str) -> bool {
        self.deregister(name)
    }

    fn get(&self, name: &str) -> Option<Arc<dyn Service<Req, Resp>>> {
        self.get(name)
    }

    fn list_names(&self) -> Vec<String> {
        self.list_names()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::service::errors::ServiceError;
    use futures::future::BoxFuture;

    struct ServiceRegistryFixture;

    impl Service<String, String> for ServiceRegistryFixture {
        fn name(&self) -> &str {
            "fixture"
        }

        fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
            Box::pin(async move { Ok(req) })
        }
    }

    fn make_registry() -> ServiceRegistry<String, String> {
        ServiceRegistry::new()
    }

    /// @covers: register
    #[test]
    fn test_register_service_is_retrievable_via_trait_happy() {
        let reg = make_registry();
        ServiceRegistryTrait::register(&reg, Arc::new(ServiceRegistryFixture));
        assert!(ServiceRegistryTrait::get(&reg, "fixture").is_some());
    }

    /// @covers: deregister
    #[test]
    fn test_deregister_absent_name_returns_false_error() {
        let reg = make_registry();
        assert!(!ServiceRegistryTrait::deregister(&reg, "ghost"));
    }

    /// @covers: len
    #[test]
    fn test_is_empty_fresh_registry_returns_true_edge() {
        let reg = make_registry();
        assert!(ServiceRegistryTrait::is_empty(&reg));
    }
}
