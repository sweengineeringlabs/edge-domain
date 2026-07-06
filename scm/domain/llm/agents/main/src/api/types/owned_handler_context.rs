use std::sync::Arc;

use edge_domain_command::CommandBus;
use edge_domain_observer::ObserverContext;
use edge_security_runtime::SecurityContext;

/// Owned counterpart to `edge_domain_handler::HandlerContext<'a>`, for callers that need
/// to build a [`HandlerContext`](edge_domain_handler::HandlerContext) later rather than
/// borrow one now — e.g. [`ConversationRunRequest`](crate::api::types::ConversationRunRequest),
/// whose per-turn `Step` bridge is constructed once and shared across all configured turns
/// via `Arc`, so it cannot hold a non-`'static` borrow.
pub struct OwnedHandlerContext {
    /// Security context threaded to every skill invocation.
    pub security: SecurityContext,
    /// Command bus threaded to every skill invocation.
    pub commands: Arc<dyn CommandBus>,
    /// Observability seam threaded to every skill invocation.
    pub observer: Box<dyn ObserverContext>,
}
