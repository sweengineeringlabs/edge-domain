//! Handler trait — the domain execution-unit contract.

use async_trait::async_trait;

use crate::api::error::HandlerError;
use crate::api::types::RequestContext;

/// A single execution unit that processes a request and returns a response.
///
/// Implement `id`, `pattern`, and `execute` — everything else has a sensible
/// default.  Override `execute_with_context` only when you need the per-request
/// auth/tenant context.
///
/// # Examples
///
/// ```rust,no_run
/// use async_trait::async_trait;
/// use edge_domain::{Handler, HandlerError};
///
/// struct GreetHandler;
///
/// #[async_trait]
/// impl Handler<String, String> for GreetHandler {
///     fn id(&self)      -> &str { "greet" }
///     fn pattern(&self) -> &str { "/api/v1/greet" }
///
///     async fn execute(&self, req: String) -> Result<String, HandlerError> {
///         if req.is_empty() {
///             return Err(HandlerError::invalid("name must not be empty"));
///         }
///         Ok(format!("Hello, {}!", req))
///     }
/// }
/// ```
#[async_trait]
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
    async fn execute(&self, req: Request) -> Result<Response, HandlerError>;

    /// Execute with per-request context.  Override when you need
    /// `ctx.subject`, `ctx.tenant_id`, or `ctx.trace_id`.
    ///
    /// Default: ignores context and calls `execute(req)`.
    async fn execute_with_context(
        &self,
        req: Request,
        _ctx: RequestContext,
    ) -> Result<Response, HandlerError> {
        self.execute(req).await
    }

    /// Return `false` when the handler is not ready to serve traffic.
    ///
    /// Default: always healthy.
    async fn health_check(&self) -> bool {
        true
    }
}
