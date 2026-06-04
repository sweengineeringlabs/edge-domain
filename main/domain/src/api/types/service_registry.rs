//! `ServiceRegistry` — thread-safe registry of [`Service`] implementations keyed by name.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::service::Service;
use crate::api::service::ServiceRegistry as ServiceRegistryTrait;

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
        Self {
            services: RwLock::new(HashMap::new()),
        }
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
    fn default() -> Self {
        Self::new()
    }
}

impl<Request, Response> ServiceRegistryTrait<Request, Response>
    for ServiceRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    fn register(&self, service: Arc<dyn Service<Request, Response>>) {
        ServiceRegistry::register(self, service);
    }

    fn deregister(&self, name: &str) -> bool {
        ServiceRegistry::deregister(self, name)
    }

    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request, Response>>> {
        ServiceRegistry::get(self, name)
    }

    fn list_names(&self) -> Vec<String> {
        ServiceRegistry::list_names(self)
    }

    fn len(&self) -> usize {
        ServiceRegistry::len(self)
    }
}
