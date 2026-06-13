//! `NoopService` — a no-operation service implementation for testing and default wiring.

/// A no-operation [`Service`](crate::api::service::traits::service::Service) that accepts
/// `()` requests and immediately returns `()`.
///
/// Useful as a sentinel or placeholder where a real service implementation is not required.
/// Constructed via [`ServiceRegistryFactory::noop_service`](crate::api::service::traits::service_registry_factory::ServiceRegistryFactory::noop_service).
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopService;
