//! Handler trait — the domain execution-unit contract.

use std::any::Any;

use async_trait::async_trait;

use crate::api::handler::request_context::RequestContext;
use crate::api::handler_error::HandlerError;

/// A single execution unit that processes a request and returns a response.
///
/// Implementations wrap a concrete domain pattern (ReAct, CoT, direct call,
/// etc.) or a specific service (auth, authz, VM lifecycle).
///
/// `as_any` enables safe downcasting when a caller needs concrete access.
#[async_trait]
pub trait Handler<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable identifier — used as the lookup key in [`HandlerRegistry`](crate::HandlerRegistry).
    fn id(&self) -> &str;

    /// Human-readable pattern or service name (e.g. `"ReAct"`, `"AuthN"`, `"KVM"`).
    fn pattern(&self) -> &str;

    /// Execute the handler with the given request.
    ///
    /// This is the required implementation entrypoint.  Handlers that do
    /// not need per-request auth context implement only this method.
    async fn execute(&self, req: Request) -> Result<Response, HandlerError>;

    /// Execute the handler with per-request context.
    ///
    /// Override this when the handler needs `ctx` (authenticated subject,
    /// tenant ID, trace ID).  The default falls through to [`execute`](Self::execute).
    ///
    /// The dispatch layer always calls `execute_with_context`, so overriding
    /// it is sufficient — no need to also override `execute`.
    async fn execute_with_context(&self, req: Request, _ctx: RequestContext) -> Result<Response, HandlerError> {
        self.execute(req).await
    }

    /// Probe whether the handler is healthy and responsive.
    async fn health_check(&self) -> bool;

    /// Downcast hook for concrete access from tests or administrative tools.
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_trait_is_object_safe() {
        fn _accept(_h: &dyn Handler<String, String>) {}
    }
}
