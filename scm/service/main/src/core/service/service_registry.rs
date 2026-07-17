//! `ServiceRegistry` trait impl for the [`ServiceRegistry`] struct — in-process registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NoopService, RegisterServiceRequest, RegisterServiceResponse,
    ServiceError, ServiceLookupRequest, ServiceLookupResponse, ServiceRegistry,
    ServiceRegistryStore, ServiceRemovalRequest, ServiceRemovalResponse, StdServiceRegistryFactory,
};

impl<Req, Resp> Default for ServiceRegistryStore<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn default() -> Self {
        ServiceRegistryStore {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<Req, Resp> ServiceRegistry for ServiceRegistryStore<Req, Resp>
where
    Req: edge_application_base::Request,
    Resp: edge_application_base::Response,
{
    type Request = Req;
    type Response = Resp;

    fn register(
        &self,
        req: &RegisterServiceRequest<Req, Resp>,
    ) -> Result<RegisterServiceResponse, ServiceError> {
        let svc = &*req.service;
        let name_req = NameRequest;
        let name_resp = svc.name(name_req)?;

        let service_arc = Arc::clone(&req.service);
        self.inner.write().insert(name_resp.name, service_arc);
        Ok(RegisterServiceResponse)
    }

    fn deregister(
        &self,
        req: &ServiceRemovalRequest,
    ) -> Result<ServiceRemovalResponse, ServiceError> {
        let was_present = self.inner.write().remove(&req.name).is_some();
        Ok(ServiceRemovalResponse { was_present })
    }

    fn get(
        &self,
        req: &ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<Req, Resp>, ServiceError> {
        let service = self.inner.read().get(&req.name).cloned();
        Ok(ServiceLookupResponse { service })
    }

    fn list_names(&self, _req: ListNamesRequest) -> Result<ListNamesResponse, ServiceError> {
        let names = self.inner.read().keys().cloned().collect();
        Ok(ListNamesResponse { names })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, ServiceError> {
        let count = self.inner.read().len();
        Ok(LenResponse { count })
    }

    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, ServiceError> {
        let empty = self.inner.read().is_empty();
        Ok(EmptinessResponse { empty })
    }

    fn default_factory() -> StdServiceRegistryFactory {
        StdServiceRegistryFactory
    }

    fn noop_service() -> NoopService {
        NoopService
    }

    fn new_store() -> ServiceRegistryStore<Req, Resp> {
        ServiceRegistryStore::default()
    }
}

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

impl<Req, Resp> RegisterServiceRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Create a new registration request.
    pub fn new(service: Arc<dyn crate::api::Service<Request = Req, Response = Resp>>) -> Self {
        Self { service }
    }
}
