//! [`ServiceLookupResponse`] — wrapper for service lookup result.

use std::sync::Arc;
use crate::api::Service;

pub struct ServiceLookupResponse<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The service if found; None otherwise.
    pub service: Option<Arc<dyn Service<Request = Req, Response = Resp>>>,
}
