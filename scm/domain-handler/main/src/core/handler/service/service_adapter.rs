//! Blanket bridges from `edge_domain_service`'s `ServiceRegistry`/`Service` traits to their
//! local `domain-handler` decoupling boundaries (SEA `no_foreign_type`).

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_service as svc;

use crate::api::HandlerError;
use crate::api::{
    ListNamesRequest, ListNamesResponse, Service, ServiceLookupRequest, ServiceLookupResponse,
    ServiceRegistry,
};

/// Bridges any concrete real `Service` implementor directly to the local [`Service`] trait.
///
/// Covers the case where the caller still holds a concrete, `Sized` service type (e.g. a
/// freshly constructed `NoopService`) and can unsize-coerce it straight to `Arc<dyn Service>`.
#[async_trait]
impl<T: svc::Service + ?Sized> Service for T {
    type Request = T::Request;
    type Response = T::Response;

    async fn execute(&self, req: Self::Request) -> Result<Self::Response, HandlerError> {
        svc::Service::execute(self, req)
            .await
            .map_err(HandlerError::from)
    }
}

/// Adapter wrapping an already-erased `Arc<dyn edge_domain_service::Service>` as a local
/// [`Service`].
///
/// Needed because an existing `Arc<dyn svc::Service>` cannot be re-coerced into
/// `Arc<dyn Service>` directly — trait-object-to-trait-object coercion isn't supported even
/// when the blanket impl above proves the underlying type satisfies both traits.
struct ServiceAdapter<Req, Resp>(Arc<dyn svc::Service<Request = Req, Response = Resp>>)
where
    Req: Send + 'static,
    Resp: Send + 'static;

#[async_trait]
impl<Req, Resp> Service for ServiceAdapter<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    async fn execute(&self, req: Req) -> Result<Resp, HandlerError> {
        self.0.execute(req).await.map_err(HandlerError::from)
    }
}

impl<T: svc::ServiceRegistry + ?Sized> ServiceRegistry for T {
    type Request = T::Request;
    type Response = T::Response;

    fn list_names(&self, _req: ListNamesRequest) -> Result<ListNamesResponse, HandlerError> {
        svc::ServiceRegistry::list_names(self, svc::ListNamesRequest)
            .map(|r| ListNamesResponse { names: r.names })
            .map_err(HandlerError::from)
    }

    fn get(
        &self,
        req: ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<Self::Request, Self::Response>, HandlerError> {
        let lookup = svc::ServiceLookupRequest { name: req.name };
        let resp = svc::ServiceRegistry::get(self, &lookup).map_err(HandlerError::from)?;
        Ok(ServiceLookupResponse {
            service: resp.service.map(|s| {
                Arc::new(ServiceAdapter(s))
                    as Arc<dyn Service<Request = Self::Request, Response = Self::Response>>
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use edge_domain_service::{
        NoopService, RegisterServiceRequest, ServiceRegistry as ForeignServiceRegistry,
        ServiceRegistryStore,
    };
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn test_list_names_bridges_registered_service_happy() {
        let store: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        store
            .register(&RegisterServiceRequest::new(Arc::new(NoopService)))
            .unwrap();
        let names = ServiceRegistry::list_names(&store, ListNamesRequest)
            .unwrap()
            .names;
        assert_eq!(names, vec!["noop".to_string()]);
    }

    #[test]
    fn test_get_missing_service_returns_none_edge() {
        let store: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        let resp = ServiceRegistry::get(
            &store,
            ServiceLookupRequest {
                name: "missing".to_string(),
            },
        )
        .unwrap();
        assert!(resp.service.is_none());
    }

    #[test]
    fn test_concrete_service_coerces_to_local_trait_object_via_blanket_impl_happy() {
        let service: Arc<dyn Service<Request = (), Response = ()>> = Arc::new(NoopService);
        assert_eq!(block_on(service.execute(())), Ok(()));
    }

    #[test]
    fn test_get_registered_service_executes_via_adapter_happy() {
        let store: ServiceRegistryStore<(), ()> = ServiceRegistryStore::default();
        store
            .register(&RegisterServiceRequest::new(Arc::new(NoopService)))
            .unwrap();
        let resp = ServiceRegistry::get(
            &store,
            ServiceLookupRequest {
                name: "noop".to_string(),
            },
        )
        .unwrap();
        let service = resp.service.expect("service should be present");
        assert!(block_on(service.execute(())).is_ok());
    }
}
