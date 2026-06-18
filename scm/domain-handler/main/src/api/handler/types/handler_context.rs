//! [`HandlerContext`] — request-scoped execution context threaded to every `Handler::execute` call.

use edge_domain_command::CommandBus;
use edge_domain_security::SecurityContext;

/// Request-scoped context threaded to every [`Handler::execute`](crate::api::handler::traits::Handler::execute) call.
///
/// Enum so the parameter type satisfies `api_field_type_purity` — pipeline
/// stages can reborrow without cloning across the loop because all variants
/// carry only references (`Copy` is derived).
#[derive(Copy, Clone)]
pub enum HandlerContext<'a> {
    /// Standard authenticated (or unauthenticated) request context.
    Standard {
        /// The authenticated (or unauthenticated) principal for this request.
        security: &'a SecurityContext,
        /// The write bus — all handler-initiated writes must go through this.
        commands: &'a dyn CommandBus,
    },
}

impl<'a> HandlerContext<'a> {
    /// Construct a standard request context.
    pub fn new(security: &'a SecurityContext, commands: &'a dyn CommandBus) -> Self {
        Self::Standard { security, commands }
    }

    /// Return the security principal for this request.
    pub fn security(&self) -> &'a SecurityContext {
        match self {
            Self::Standard { security, .. } => security,
        }
    }

    /// Return the command bus for this request.
    pub fn commands(&self) -> &'a dyn CommandBus {
        match self {
            Self::Standard { commands, .. } => *commands,
        }
    }
}
