//! `ServiceRegistry` trait — a named registry of [`Service`] implementations.

use std::sync::Arc;

use super::service::Service;

/// A registry that maps service names to [`Service`] implementations.
///
/// Implementations must be thread-safe. The canonical implementation is
/// [`crate::api::service::types::ServiceRegistry`].
pub trait ServiceRegistry<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Register a service under its reported name.
    fn register(&self, service: Arc<dyn Service<Request, Response>>);

    /// Remove the service with the given name. Returns `true` if it was present.
    fn deregister(&self, name: &str) -> bool;

    /// Look up a service by name.
    fn get(&self, name: &str) -> Option<Arc<dyn Service<Request, Response>>>;

    /// Return the names of all registered services.
    fn list_names(&self) -> Vec<String>;

    /// Return the number of registered services.
    fn len(&self) -> usize;

    /// Return `true` when no services are registered.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
