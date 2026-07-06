//! [`BridgeRequest`] — request for [`RegistryBridge::bridge`](crate::api::handler::traits::RegistryBridge::bridge).

use crate::api::handler::traits::handler_registry::HandlerRegistry;
use edge_domain_service::ServiceRegistry as ServiceRegistryTrait;

/// Request to transfer every service in `src` into `dst` as a handler.
pub struct BridgeRequest<'a, Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The service registry to read services from.
    pub src: &'a dyn ServiceRegistryTrait<Request = Req, Response = Resp>,
    /// The handler registry to write bridged handlers into.
    pub dst: &'a dyn HandlerRegistry<Request = Req, Response = Resp>,
}
