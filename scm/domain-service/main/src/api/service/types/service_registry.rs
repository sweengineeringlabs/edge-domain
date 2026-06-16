//! API-layer type for the in-process service registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::service::traits::service::Service;

/// A thread-safe, in-process registry mapping service names to [`Service`] implementations.
///
/// The struct provides inherent methods that satisfy the
/// [`ServiceRegistry`](crate::api::service::traits::service_registry::ServiceRegistry) trait
/// contract. The trait implementation lives in `core::service::service_registry`.
pub struct ServiceRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    inner: RwLock<HashMap<String, Arc<dyn Service<Request = Req, Response = Resp>>>>,
}

impl<Req, Resp> ServiceRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// Register a service under its reported name.
    pub fn register(&self, service: Arc<dyn Service<Request = Req, Response = Resp>>) {
        self.inner.write().insert(service.name().to_owned(), service);
    }

    /// Remove the service with the given name. Returns `true` if it was present.
    pub fn deregister(&self, name: &str) -> bool {
        self.inner.write().remove(name).is_some()
    }

    /// Look up a service by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Service<Request = Req, Response = Resp>>> {
        self.inner.read().get(name).cloned()
    }

    /// Return the names of all registered services.
    pub fn list_names(&self) -> Vec<String> {
        self.inner.read().keys().cloned().collect()
    }

    /// Return the number of registered services.
    pub fn len(&self) -> usize {
        self.inner.read().len()
    }

    /// Return `true` when no services are registered.
    pub fn is_empty(&self) -> bool {
        self.inner.read().is_empty()
    }
}

impl<Req, Resp> Default for ServiceRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
