//! `RegistryBridge` impl for [`StdRegistryBridge`] — bridges a `ServiceRegistry` into a `HandlerRegistry`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_service::ListNamesRequest;
use edge_domain_service::Service;
use edge_domain_service::ServiceLookupRequest;

use crate::api::BridgeRequest;
use crate::api::BridgeResponse;
use crate::api::ExecutionRequest;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::IdRequest;
use crate::api::IdResponse;
use crate::api::RegisterHandlerRequest;
use crate::api::RegistryBridge;
use crate::api::StdRegistryBridge;

struct StdRegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    id: String,
    inner: Arc<dyn Service<Request = Req, Response = Resp>>,
}

impl<Req, Resp> StdRegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn new(id: String, inner: Arc<dyn Service<Request = Req, Response = Resp>>) -> Self {
        Self { id, inner }
    }
}

#[async_trait]
impl<Req, Resp> Handler for StdRegistryBridgeHandler<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }

    #[allow(clippy::missing_errors_doc)]
    async fn execute(&self, req: ExecutionRequest<'_, Req>) -> Result<Resp, HandlerError> {
        self.inner
            .execute(req.req)
            .await
            .map_err(HandlerError::from)
    }
}

impl RegistryBridge for StdRegistryBridge {
    fn bridge<Req, Resp>(
        &self,
        req: BridgeRequest<'_, Req, Resp>,
    ) -> Result<BridgeResponse, HandlerError>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        let names = req.src.list_names(ListNamesRequest)?.names;
        let count = names.len();
        for name in names {
            let lookup = ServiceLookupRequest { name: name.clone() };
            if let Some(svc) = req.src.get(&lookup)?.service {
                req.dst.register(RegisterHandlerRequest::new(Arc::new(
                    StdRegistryBridgeHandler::new(name, svc),
                )))?;
            }
        }
        Ok(BridgeResponse { transferred: count })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityServices};
    use edge_domain_service::{
        NoopService, RegisterServiceRequest as RegisterServiceRequestSvc,
        ServiceRegistry as ServiceRegistryTraitSvc, ServiceRegistryStore,
    };
    use futures::executor::block_on;

    use super::StdRegistryBridgeHandler;
    use crate::api::{
        BridgeRequest, ExecutionRequest, Handler, HandlerContext, HandlerLookupRequest,
        HandlerRegistry, IdRequest, InProcessHandlerRegistry, LenRequest, RegistryBridge,
        StdRegistryBridge,
    };

    #[test]
    fn test_new_wraps_inner_service_and_preserves_id_happy() {
        let handler = StdRegistryBridgeHandler::new("svc-1".to_string(), Arc::new(NoopService));
        assert_eq!(handler.id(IdRequest).unwrap().id, "svc-1");

        let security = SecurityServices::unauthenticated();
        let bus = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &bus,
            observer: observer.as_ref(),
        };
        assert!(block_on(handler.execute(ExecutionRequest { req: (), ctx: &ctx })).is_ok());
    }

    #[test]
    fn test_bridge_populates_handler_registry_happy() {
        let src: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        src.register(&RegisterServiceRequestSvc::new(Arc::new(NoopService)))
            .unwrap();
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        let result = StdRegistryBridge.bridge(BridgeRequest {
            src: &src,
            dst: &dst,
        });
        assert_eq!(result.unwrap().transferred, 1);
        assert_eq!(dst.len(LenRequest).unwrap().count, 1);
        let handler = dst
            .get(HandlerLookupRequest {
                id: "noop".to_string(),
            })
            .unwrap()
            .handler;
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "noop");
    }

    #[test]
    fn test_bridge_empty_registry_returns_zero_error() {
        let src: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        let result = StdRegistryBridge.bridge(BridgeRequest {
            src: &src,
            dst: &dst,
        });
        assert_eq!(result.unwrap().transferred, 0);
        assert_eq!(dst.len(LenRequest).unwrap().count, 0);
    }

    #[test]
    fn test_bridge_registered_handler_has_correct_id_edge() {
        let src: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        src.register(&RegisterServiceRequestSvc::new(Arc::new(NoopService)))
            .unwrap();
        let dst = InProcessHandlerRegistry::<(), ()>::default();
        StdRegistryBridge
            .bridge(BridgeRequest {
                src: &src,
                dst: &dst,
            })
            .unwrap();
        let handler = dst
            .get(HandlerLookupRequest {
                id: "noop".to_string(),
            })
            .unwrap()
            .handler;
        assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "noop");
    }
}
