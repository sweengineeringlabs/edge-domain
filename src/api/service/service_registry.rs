//! `ServiceRegistry` — thread-safe registry of [`Service`] implementations keyed by name.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::service::Service;

/// Registry of [`Service`] instances keyed by [`Service::name`].
///
/// Concurrency: guarded by a `parking_lot::RwLock` — lookups proceed in
/// parallel while registration and deregistration are serialized.
pub struct ServiceRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    pub(crate) services: RwLock<HashMap<String, Arc<dyn Service<Request, Response>>>>,
}

impl<Request, Response> ServiceRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self { services: RwLock::new(HashMap::new()) }
    }

    /// Register a service, replacing any existing entry with the same name.
    pub fn register(&self, service: Arc<dyn Service<Request, Response>>) {
        let name = service.name().to_string();
        self.services.write().insert(name, service);
    }

    /// Deregister the service with the given name. Returns `true` if removed.
    pub fn deregister(&self, name: &str) -> bool {
        self.services.write().remove(name).is_some()
    }

    /// Look up a service by name. Returns `None` if not registered.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Service<Request, Response>>> {
        self.services.read().get(name).cloned()
    }

    /// Snapshot of registered service names. Order is unspecified.
    pub fn list_names(&self) -> Vec<String> {
        self.services.read().keys().cloned().collect()
    }

    /// Number of currently registered services.
    pub fn len(&self) -> usize {
        self.services.read().len()
    }

    /// Whether the registry has no services.
    pub fn is_empty(&self) -> bool {
        self.services.read().is_empty()
    }
}

impl<Request, Response> Default for ServiceRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::service::ServiceError;
    use async_trait::async_trait;

    struct ServiceStub { name: String }
    #[async_trait]
    impl Service<String, String> for ServiceStub {
        fn name(&self) -> &str { &self.name }
        async fn execute(&self, req: String) -> Result<String, ServiceError> { Ok(req) }
    }
    fn stub(name: &str) -> Arc<dyn Service<String, String>> {
        Arc::new(ServiceStub { name: name.to_string() })
    }

    /// @covers: register
    #[test]
    fn test_register_stores_service_retrievable_by_name() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        reg.register(stub("svc-a"));
        assert!(reg.get("svc-a").is_some());
    }

    /// @covers: get
    #[test]
    fn test_get_returns_none_for_unregistered_name() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        assert!(reg.get("missing").is_none());
    }

    /// @covers: deregister
    #[test]
    fn test_deregister_removes_service_and_returns_true() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        reg.register(stub("svc-a"));
        assert!(reg.deregister("svc-a"));
        assert!(reg.get("svc-a").is_none());
    }

    /// @covers: list_names
    #[test]
    fn test_list_names_returns_all_registered_names() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        reg.register(stub("a"));
        reg.register(stub("b"));
        let mut names = reg.list_names();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }

    /// @covers: len
    #[test]
    fn test_len_returns_correct_count() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        assert_eq!(reg.len(), 0);
        reg.register(stub("a"));
        assert_eq!(reg.len(), 1);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_reflects_registry_state() {
        let reg: ServiceRegistry<String, String> = ServiceRegistry::new();
        assert!(reg.is_empty());
        reg.register(stub("a"));
        assert!(!reg.is_empty());
    }
}
