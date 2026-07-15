//! [`RegisterServiceRequest`] — service registration request.

use crate::api::Service;
use std::sync::Arc;

/// Request to register a service in the registry.
///
/// Encapsulates a service instance with a private Arc field,
/// preventing external code from depending on the Arc wrapping pattern.
/// Construction methods are in core/service/register_service_request.rs.
pub struct RegisterServiceRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) service: Arc<dyn Service<Request = Req, Response = Resp>>,
}
