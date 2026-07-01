//! [`RegisterServiceRequest`] — service registration request.

use crate::api::Service;
use std::sync::Arc;

/// Request to register a service in the registry.
pub struct RegisterServiceRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The service to register.
    pub service: Arc<dyn Service<Request = Req, Response = Resp>>,
}
