//! [`ServiceRegistry`] trait — contract for service instance registries.

use std::sync::Arc;

use crate::api::service::Service;

/// Thread-safe registry of [`Service`] instances keyed by name.
pub trait ServiceRegistry: Send + Sync {
    /// The request type accepted by services in this registry.
    type Request: Send + 'static;
    /// The response type produced by services in this registry.
    type Response: Send + 'static;

    /// Register a service, replacing any existing entry with the same name.
    fn register(&self, service: Arc<dyn Service<Request = Self::Request, Response = Self::Response>>);

    /// Deregister the service with the given name. Returns `true` if removed.
    fn deregister(&self, name: &str) -> bool;

    /// Look up a service by name. Returns `None` if not registered.
    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request = Self::Request, Response = Self::Response>>>;

    /// Snapshot of registered service names. Order is unspecified.
    fn list_names(&self) -> Vec<String>;

    /// Number of currently registered services.
    fn len(&self) -> usize;

    /// Whether the registry has no services.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub use crate::api::service::types::ServiceRegistry as ServiceRegistryImpl;
