//! RegisterServiceRequest constructors (impl belongs in core, not api).

use crate::api::{RegisterServiceRequest, Service};
use std::sync::Arc;

impl<Req, Resp> RegisterServiceRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Create a new registration request.
    pub fn new(service: Arc<dyn Service<Request = Req, Response = Resp>>) -> Self {
        Self { service }
    }
}
