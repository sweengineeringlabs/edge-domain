//! `ServiceRegistry` trait impl for the [`ServiceRegistry`] struct — in-process registry.

use std::sync::Arc;

use crate::api::ServiceRegistryTrait as ServiceRegistry;
use crate::api::ServiceRegistry as ServiceRegistryStore;
use crate::api::Service;

impl<Req, Resp> ServiceRegistry for ServiceRegistryStore<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    fn register(&self, service: Arc<dyn Service<Request = Req, Response = Resp>>) {
        self.register(service);
    }

    fn deregister(&self, name: &str) -> bool {
        self.deregister(name)
    }

    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request = Req, Response = Resp>>> {
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
    use std::sync::Arc;

    use futures::future::BoxFuture;

    use crate::api::ServiceError;
    use crate::api::ServiceRegistryTrait;
    use crate::api::ServiceRegistry;
    use crate::api::Service;

    struct ServiceRegistryFixture;

    impl Service for ServiceRegistryFixture {
        type Request = String;
        type Response = String;

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
        let svc = ServiceRegistryTrait::get(&reg, "fixture").expect("service must be registered");
        assert_eq!(svc.name(), "fixture");
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
