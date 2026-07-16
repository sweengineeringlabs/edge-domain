//! [`RegistryBridge`] — bulk-transfers services from a [`ServiceRegistry`] into a [`HandlerRegistry`].

use crate::api::handler::errors::HandlerError;
use crate::api::handler::dto::{BridgeRequest, BridgeResponse};
use crate::api::handler::std_registry_bridge::StdRegistryBridge;

/// Bridges a [`ServiceRegistry`](super::ServiceRegistry) into a
/// [`HandlerRegistry`](super::traits::HandlerRegistry) by wrapping each registered
/// service as a handler.
///
/// The canonical implementation is [`StdRegistryBridge`], obtained via
/// [`default_bridge`](RegistryBridge::default_bridge).
pub trait RegistryBridge: Send + Sync {
    /// Transfer every service in `req.src` into `req.dst` as a [`Handler`](super::traits::Handler).
    ///
    /// Each service is wrapped so that its `name()` becomes the handler `id()`,
    /// and calls to `Handler::execute` delegate to `Service::execute`.
    /// Returns the number of services transferred.
    fn bridge<Req, Resp>(
        &self,
        req: BridgeRequest<'_, Req, Resp>,
    ) -> Result<BridgeResponse, HandlerError>
    where
        Req: edge_application_base::Request,
        Resp: edge_application_base::Response;

    /// Construct the standard zero-config bridge implementation.
    fn default_bridge() -> StdRegistryBridge
    where
        Self: Sized,
    {
        StdRegistryBridge
    }
}
