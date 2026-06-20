//! [`ServiceRegistryBootstrap`] — constructor contract for `ServiceRegistry` implementations.

use crate::api::service::types::StdServiceRegistryFactory;
use crate::api::service::types::NoopService;
use crate::api::service::types::ServiceRegistry;

/// Bootstrap trait for constructing [`ServiceRegistry`] instances.
pub trait ServiceRegistryBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "service_registry"
    }

    /// Construct a new, empty [`ServiceRegistry`].
    fn new_registry<Req, Resp>() -> ServiceRegistry<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
        Self: Sized,
    {
        ServiceRegistry::new()
    }

    /// Construct a [`NoopService`] — a no-operation sentinel service.
    fn noop_service() -> NoopService
    where
        Self: Sized,
    {
        NoopService
    }

    /// Return the [`StdServiceRegistryFactory`] — the standard zero-config factory.
    fn default_factory() -> StdServiceRegistryFactory
    where
        Self: Sized,
    {
        StdServiceRegistryFactory
    }
}
