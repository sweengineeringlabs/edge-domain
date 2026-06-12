//! `Handler` trait — an async request/response execution unit.

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::RequestContext;

/// An async request/response execution unit identified by an id and pattern.
#[async_trait]
pub trait Handler<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable identifier for this handler.
    fn id(&self) -> &str {
        "handler"
    }

    /// Route pattern this handler matches.
    fn pattern(&self) -> &str {
        ""
    }

    /// Execute the handler with the given request.
    async fn execute(&self, req: Request) -> Result<Response, HandlerError>;

    /// Execute the handler with an explicit [`RequestContext`].
    ///
    /// The default implementation ignores the context and delegates to [`execute`](Handler::execute).
    async fn execute_with_context(
        &self,
        req: Request,
        _ctx: RequestContext,
    ) -> Result<Response, HandlerError> {
        self.execute(req).await
    }

    /// Return `true` if the handler is healthy and able to process requests.
    async fn health_check(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysOk;

    #[async_trait]
    impl Handler<String, String> for AlwaysOk {
        async fn execute(&self, req: String) -> Result<String, HandlerError> {
            Ok(req)
        }
    }

    struct AlwaysFail;

    #[async_trait]
    impl Handler<String, String> for AlwaysFail {
        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Err(HandlerError::ExecutionFailed("fail".into()))
        }
    }

    #[tokio::test]
    async fn test_execute_ok_handler_returns_response_happy() {
        assert!(AlwaysOk.execute("hi".into()).await.is_ok());
    }

    #[tokio::test]
    async fn test_execute_failing_handler_returns_err_error() {
        assert!(AlwaysFail.execute("hi".into()).await.is_err());
    }

    #[tokio::test]
    async fn test_id_default_returns_handler_edge() {
        assert_eq!(AlwaysOk.id(), "handler");
    }

    #[tokio::test]
    async fn test_pattern_default_returns_empty_edge() {
        assert_eq!(AlwaysOk.pattern(), "");
    }

    #[tokio::test]
    async fn test_health_check_default_returns_true_happy() {
        assert!(AlwaysOk.health_check().await);
    }

    #[tokio::test]
    async fn test_execute_with_context_delegates_to_execute_happy() {
        let ctx = RequestContext::default();
        let result = AlwaysOk
            .execute_with_context("hello".into(), ctx)
            .await;
        assert_eq!(result.unwrap(), "hello");
    }
}
