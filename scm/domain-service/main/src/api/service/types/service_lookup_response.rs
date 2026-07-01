//! [`ServiceLookupResponse`] — wrapper for service lookup result.

use crate::api::Service;
use std::sync::Arc;

pub struct ServiceLookupResponse<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The service if found; None otherwise.
    pub service: Option<Arc<dyn Service<Request = Req, Response = Resp>>>,
}
