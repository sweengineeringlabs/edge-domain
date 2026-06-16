//! [`ServiceRegistryFactory`] — constructor contract for `ServiceRegistry` implementations.

use crate::api::service::types::StdServiceRegistryFactory;
use crate::api::service::types::NoopService;
use crate::api::service::types::ServiceRegistry;

/// Factory trait for constructing [`ServiceRegistry`] instances.
pub trait ServiceRegistryFactory {
    /// Construct a new, empty [`ServiceRegistry`].
    fn new_registry<Req, Resp>() -> ServiceRegistry<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        ServiceRegistry::new()
    }

    /// Construct a [`NoopService`] — a no-operation sentinel service.
    fn noop_service() -> NoopService {
        NoopService
    }

    /// Return the [`StdServiceRegistryFactory`] — the standard zero-config factory.
    fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }
}
