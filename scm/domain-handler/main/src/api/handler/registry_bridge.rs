//! [`RegistryBridge`] — bulk-transfers services from a [`ServiceRegistry`] into a [`HandlerRegistry`].

use crate::api::handler::traits::handler_registry::HandlerRegistry;
use crate::api::handler::types::std_registry_bridge::StdRegistryBridge;
use edge_domain_service::ServiceRegistryTrait;

/// Bridges a [`ServiceRegistry`](edge_domain_service::ServiceRegistryTrait) into a
/// [`HandlerRegistry`] by wrapping each registered service as a handler.
///
/// The canonical implementation is [`StdRegistryBridge`], obtained via
/// [`default_bridge`](RegistryBridge::default_bridge).
pub trait RegistryBridge: Send + Sync {
    /// Transfer every service in `src` into `dst` as a [`Handler`](super::traits::Handler).
    ///
    /// Each service is wrapped so that its `name()` becomes the handler `id()`,
    /// and calls to `Handler::execute` delegate to `Service::execute`.
    /// Returns the number of services transferred.
    fn bridge<Req, Resp>(
        &self,
        src: &dyn ServiceRegistryTrait<Request = Req, Response = Resp>,
        dst: &dyn HandlerRegistry<Request = Req, Response = Resp>,
    ) -> usize
    where
        Req: Send + 'static,
        Resp: Send + 'static;

    /// Construct the standard zero-config bridge implementation.
    fn default_bridge() -> StdRegistryBridge
    where
        Self: Sized,
    {
        StdRegistryBridge
    }
}
