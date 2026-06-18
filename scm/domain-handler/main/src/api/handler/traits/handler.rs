//! `Handler` trait — an async request/response execution unit.

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::HandlerContext;

/// An async request/response execution unit identified by an id and pattern.
#[async_trait]
pub trait Handler: Send + Sync {
    /// The request type this handler accepts.
    type Request: Send + 'static;

    /// The response type this handler produces.
    type Response: Send + 'static;

    /// Stable identifier for this handler.
    fn id(&self) -> &str {
        "handler"
    }

    /// Route pattern this handler matches.
    fn pattern(&self) -> &str {
        ""
    }

    /// Execute the handler with the given request and request-scoped context.
    async fn execute(
        &self,
        req: Self::Request,
        ctx: HandlerContext<'_>,
    ) -> Result<Self::Response, HandlerError>;

    /// Return `true` if the handler is healthy and able to process requests.
    async fn health_check(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;

    struct AlwaysOk;

    #[async_trait]
    impl Handler for AlwaysOk {
        type Request = String;
        type Response = String;

        async fn execute(
            &self,
            req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Ok(req)
        }
    }

    struct AlwaysFail;

    #[async_trait]
    impl Handler for AlwaysFail {
        type Request = String;
        type Response = String;

        async fn execute(
            &self,
            _req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Err(HandlerError::ExecutionFailed("fail".into()))
        }
    }

    #[tokio::test]
    async fn test_execute_ok_handler_returns_response_happy() {
        let security = SecurityContext::unauthenticated();
        let bus = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &bus);
        assert!(AlwaysOk.execute("hi".into(), ctx).await.is_ok());
    }

    #[tokio::test]
    async fn test_execute_failing_handler_returns_err_error() {
        let security = SecurityContext::unauthenticated();
        let bus = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &bus);
        assert!(AlwaysFail.execute("hi".into(), ctx).await.is_err());
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
}
