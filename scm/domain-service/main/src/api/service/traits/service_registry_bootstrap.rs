//! [`ServiceRegistryBootstrap`] — constructor contract for `ServiceRegistry` implementations.

use crate::api::service::types::StdServiceRegistryFactory;
use crate::api::service::types::NoopService;
use crate::api::service::types::ServiceRegistryStore;

/// Bootstrap trait for constructing [`ServiceRegistry`] instances.
///
/// Static factory methods only — used for factory setup.
pub trait ServiceRegistryBootstrap {
    /// Construct a new, empty [`ServiceRegistry`].
    fn new_registry<Req, Resp>() -> ServiceRegistryStore<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
        Self: Sized,
    {
        ServiceRegistryStore::default()
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
