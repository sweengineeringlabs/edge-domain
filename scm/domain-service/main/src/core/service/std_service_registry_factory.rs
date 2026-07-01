//! `StdServiceRegistryFactory` trait impl — default factory implementation.

use crate::api::{NoopService, ServiceRegistryStore, StdServiceRegistryFactory};

impl StdServiceRegistryFactory {
    /// Construct a new, empty service registry.
    pub fn new_registry<Req, Resp>() -> ServiceRegistryStore<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        ServiceRegistryStore::default()
    }

    /// Construct a [`NoopService`] — a no-operation sentinel service.
    pub fn noop_service() -> NoopService {
        NoopService
    }

    /// Return the [`StdServiceRegistryFactory`] — the standard zero-config factory.
    pub fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }
}
