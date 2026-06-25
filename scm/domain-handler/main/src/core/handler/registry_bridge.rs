//! `RegistryBridge` impl for [`StdRegistryBridge`] — bridges a `ServiceRegistry` into a `HandlerRegistry`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_service::Service;
use edge_domain_service::ServiceRegistryTrait;

use crate::api::Handler;
use crate::api::HandlerContext;
use crate::api::HandlerError;
use crate::api::HandlerRegistry;
use crate::api::RegistryBridge;
use crate::api::StdRegistryBridge;

struct RegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    id: String,
    inner: Arc<dyn Service<Request = Req, Response = Resp>>,
}

impl<Req, Resp> RegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn new(id: String, inner: Arc<dyn Service<Request = Req, Response = Resp>>) -> Self {
        Self { id, inner }
    }
}

#[async_trait]
impl<Req, Resp> Handler for RegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    fn id(&self) -> &str {
        &self.id
    }

    #[allow(clippy::missing_errors_doc)]
    async fn execute(&self, req: Req, _ctx: HandlerContext<'_>) -> Result<Resp, HandlerError> {
        self.inner.execute(req).await.map_err(HandlerError::from)
    }
}

impl RegistryBridge for StdRegistryBridge {
    fn bridge<Req, Resp>(
        &self,
        src: &dyn ServiceRegistryTrait<Request = Req, Response = Resp>,
        dst: &dyn HandlerRegistry<Request = Req, Response = Resp>,
    ) -> usize
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        let names = src.list_names();
        let count = names.len();
        for name in names {
            if let Some(svc) = src.get(&name) {
                dst.register(Arc::new(RegistryBridgeHandler::new(name, svc)));
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain_service::{NoopService, ServiceRegistry, ServiceRegistryTrait};

    use crate::api::{HandlerRegistry, InProcessHandlerRegistry, RegistryBridge, StdRegistryBridge};

    #[test]
    fn test_bridge_populates_handler_registry_happy() {
        let src = ServiceRegistry::<(), ()>::default();
        src.register(Arc::new(NoopService));
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        let count = StdRegistryBridge.bridge(&src, &dst);
        assert_eq!(count, 1);
        assert_eq!(dst.len(), 1);
        assert!(dst.get("noop").is_some());
        assert_eq!(dst.get("noop").unwrap().id(), "noop");
    }

    #[test]
    fn test_bridge_empty_registry_returns_zero_error() {
        let src = ServiceRegistry::<(), ()>::default();
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        let count = StdRegistryBridge.bridge(&src, &dst);
        assert_eq!(count, 0);
        assert_eq!(dst.len(), 0);
    }

    #[test]
    fn test_bridge_registered_handler_has_correct_id_edge() {
        let src = ServiceRegistry::<(), ()>::default();
        src.register(Arc::new(NoopService));
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        StdRegistryBridge.bridge(&src, &dst);
        assert_eq!(dst.get("noop").unwrap().id(), "noop");
    }
}
