//! `ServiceRegistry` trait — a named registry of [`Service`] implementations.

use std::sync::Arc;

use super::service::Service;

/// A registry that maps service names to [`Service`] implementations.
///
/// Implementations must be thread-safe. The canonical implementation is
/// [`crate::api::service::types::ServiceRegistry`].
pub trait ServiceRegistry: Send + Sync {
    /// The request type accepted by services in this registry.
    type Request: Send + 'static;
    /// The response type produced by services in this registry.
    type Response: Send + 'static;

    /// Register a service under its reported name.
    fn register(
        &self,
        service: Arc<dyn Service<Request = Self::Request, Response = Self::Response>>,
    );

    /// Remove the service with the given name. Returns `true` if it was present.
    fn deregister(&self, name: &str) -> bool;

    /// Look up a service by name.
    fn get(
        &self,
        name: &str,
    ) -> Option<Arc<dyn Service<Request = Self::Request, Response = Self::Response>>>;

    /// Return the names of all registered services.
    fn list_names(&self) -> Vec<String>;

    /// Return the number of registered services.
    fn len(&self) -> usize;

    /// Return `true` when no services are registered.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
