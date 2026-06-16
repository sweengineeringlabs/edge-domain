//! [`HandlerContext`] — request-scoped execution context threaded to every `Handler::execute` call.

use edge_domain_command::CommandBus;
use edge_domain_security::SecurityContext;

/// Request-scoped context threaded to every [`Handler::execute`](crate::api::handler::traits::Handler::execute) call.
///
/// Both fields are references so `HandlerContext` is `Copy` — pipeline stages
/// can reborrow without cloning across the loop.
#[derive(Copy, Clone)]
pub struct HandlerContext<'a> {
    /// The authenticated (or unauthenticated) principal for this request.
    pub security: &'a SecurityContext,
    /// The write bus — all handler-initiated writes must go through this.
    pub commands: &'a dyn CommandBus,
}
