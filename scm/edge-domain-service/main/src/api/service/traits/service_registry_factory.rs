//! [`ServiceRegistryFactory`] — constructor contract for `ServiceRegistry` implementations.

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
}
