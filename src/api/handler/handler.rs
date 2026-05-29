//! Handler trait — the domain execution-unit contract.

use futures::future::BoxFuture;

use crate::api::error::HandlerError;
use crate::api::types::RequestContext;

/// A single execution unit that processes a request and returns a response.
///
/// Implement `id`, `pattern`, and `execute` — everything else has a sensible
/// default.  Override `execute_with_context` only when you need the per-request
/// auth/tenant context.
///
/// ```rust,ignore
/// impl Handler<MyReq, MyResp> for MyHandler {
///     fn id(&self)      -> &str { "my-handler" }
///     fn pattern(&self) -> &str { "/api/v1/thing" }
///     fn execute<'a>(&'a self, req: MyReq) -> BoxFuture<'a, Result<MyResp, HandlerError>> {
///         Box::pin(async move {
///             // business logic
///         })
///     }
/// }
/// ```
pub trait Handler<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable identifier used as the lookup key in [`HandlerRegistry`](crate::HandlerRegistry).
    fn id(&self) -> &str {
        "handler"
    }

    /// URL pattern or service name used for routing (e.g. `"/api/v1/users/:id"`).
    fn pattern(&self) -> &str {
        ""
    }

    /// Execute the handler.  Required.
    ///
    /// Handlers that do not need per-request auth context implement only this
    /// method.  The dispatch layer calls [`execute_with_context`](Self::execute_with_context),
    /// which defaults to forwarding here.
    fn execute(&self, req: Request) -> BoxFuture<'_, Result<Response, HandlerError>>;

    /// Execute with per-request context.  Override when you need
    /// `ctx.subject`, `ctx.tenant_id`, or `ctx.trace_id`.
    ///
    /// Default: ignores context and calls `execute(req)`.
    fn execute_with_context(
        &self,
        req: Request,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, Result<Response, HandlerError>> {
        self.execute(req)
    }

    /// Return `false` when the handler is not ready to serve traffic.
    ///
    /// Default: always healthy.
    fn health_check(&self) -> BoxFuture<'_, bool> {
        Box::pin(async { true })
    }
}
