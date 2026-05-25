//! [`ServiceRegistry`] trait — contract for service instance registries.

use std::sync::Arc;

use crate::api::service::Service;

/// Thread-safe registry of [`Service`] instances keyed by name.
pub trait ServiceRegistry<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Register a service, replacing any existing entry with the same name.
    fn register(&self, service: Arc<dyn Service<Request, Response>>);

    /// Deregister the service with the given name. Returns `true` if removed.
    fn deregister(&self, name: &str) -> bool;

    /// Look up a service by name. Returns `None` if not registered.
    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request, Response>>>;

    /// Snapshot of registered service names. Order is unspecified.
    fn list_names(&self) -> Vec<String>;

    /// Number of currently registered services.
    fn len(&self) -> usize;

    /// Whether the registry has no services.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
