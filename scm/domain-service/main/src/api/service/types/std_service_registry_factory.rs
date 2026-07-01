//! `StdServiceRegistryFactory` — default concrete factory for service registry construction.

use super::ServiceRegistryStore;
use crate::core::NoopService;

/// The default concrete factory for constructing [`ServiceRegistry`](super::ServiceRegistry) instances.
///
/// Provides static factory methods for creating registries, noop services, and related instances.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StdServiceRegistryFactory;

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
