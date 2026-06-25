//! [`IntoHandler`] — converts a [`Service`](edge_domain_service::Service) into a dispatch-pipeline [`Handler`].

use crate::api::handler::service_handler::ServiceHandler as ServiceHandlerTrait;
use crate::api::handler::traits::handler::Handler;

/// Extension trait: convert a domain [`Service`](edge_domain_service::Service) into a
/// dispatch-pipeline [`Handler`].
///
/// Implemented blanket for all `S: Service + Send + Sync`. Callers use
/// `svc.into_handler()` — the only sanctioned way to register a domain service
/// into the dispatch pipeline.
pub trait IntoHandler {
    /// The service request type.
    type Request;
    /// The service response type.
    type Response;

    /// Wrap `self` as a [`Handler`] that also satisfies the [`ServiceBackedHandler`](ServiceBackedHandlerTrait)
    /// supertrait contract.
    fn into_handler(
        self,
    ) -> impl Handler<Request = Self::Request, Response = Self::Response> + ServiceHandlerTrait;
}
