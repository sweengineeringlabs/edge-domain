//! [`ServiceLookupResponse`] — response for [`ServiceRegistry::get`](crate::api::handler::traits::ServiceRegistry::get).

use std::sync::Arc;

use crate::api::handler::traits::service::Service;

/// The service found for the requested name, if any.
pub struct ServiceLookupResponse<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The service registered under the requested name, if present.
    pub service: Option<Arc<dyn Service<Request = Req, Response = Resp>>>,
}
