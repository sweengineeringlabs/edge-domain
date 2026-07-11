//! [`HandlerContext`] — request-scoped execution context threaded to every `Handler::execute` call.

use crate::api::handler::traits::{CommandBus, ObserverContext, SecurityPrincipal};

/// Request-scoped context threaded to every [`Handler::execute`](crate::api::handler::traits::Handler::execute) call.
///
/// All fields are public references — callers construct with a struct literal and read fields
/// directly. Construction and accessor methods live in `core::handler::handler_context`.
#[derive(Copy, Clone)]
pub struct HandlerContext<'a> {
    /// The authenticated (or unauthenticated) principal for this request.
    pub security: &'a dyn SecurityPrincipal,
    /// The write bus — all handler-initiated writes must go through this.
    pub commands: &'a dyn CommandBus,
    /// Observability seam — tracer, log drain, and metric registry for this request.
    pub observer: &'a dyn ObserverContext,
}
