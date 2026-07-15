//! `ServiceRegistry` trait — a named registry of [`Service`] implementations.

use crate::api::service::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NoopService, RegisterServiceRequest, RegisterServiceResponse, ServiceError,
    ServiceLookupRequest, ServiceLookupResponse, ServiceRegistryStore, ServiceRemovalRequest,
    ServiceRemovalResponse, StdServiceRegistryFactory,
};

/// A registry that maps service names to [`Service`] implementations.
///
/// Implementations must be thread-safe. The canonical implementation is
/// [`crate::api::service::service_registry_store::ServiceRegistryStore`].
pub trait ServiceRegistry: Send + Sync {
    /// The request type accepted by services in this registry.
    type Request: Send + 'static;
    /// The response type produced by services in this registry.
    type Response: Send + 'static;

    /// Register a service under its reported name.
    fn register(
        &self,
        req: &RegisterServiceRequest<Self::Request, Self::Response>,
    ) -> Result<RegisterServiceResponse, ServiceError>;

    /// Remove the service with the given name.
    fn deregister(
        &self,
        req: &ServiceRemovalRequest,
    ) -> Result<ServiceRemovalResponse, ServiceError>;

    /// Look up a service by name.
    fn get(
        &self,
        req: &ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<Self::Request, Self::Response>, ServiceError>;

    /// Return the names of all registered services.
    fn list_names(&self, req: ListNamesRequest) -> Result<ListNamesResponse, ServiceError>;

    /// Return the number of registered services.
    fn len(&self, req: LenRequest) -> Result<LenResponse, ServiceError>;

    /// Return `true` when no services are registered.
    fn is_empty(&self, req: EmptinessRequest) -> Result<EmptinessResponse, ServiceError>;

    /// Create a default empty registry (for factory support).
    fn default_factory() -> StdServiceRegistryFactory
    where
        Self: Sized;

    /// Provide a noop service for testing.
    fn noop_service() -> NoopService
    where
        Self: Sized;

    /// Construct a fresh, empty in-process registry store bound to this
    /// trait's request/response types.
    fn new_store() -> ServiceRegistryStore<Self::Request, Self::Response>
    where
        Self: Sized;
}
