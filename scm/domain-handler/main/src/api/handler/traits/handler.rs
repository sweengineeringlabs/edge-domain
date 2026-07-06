//! `Handler` trait — an async request/response execution unit.

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    ExecutionRequest, HealthCheckRequest, HealthCheckResponse, IdRequest, IdResponse,
    PatternRequest, PatternResponse,
};

/// An async request/response execution unit identified by an id and pattern.
#[async_trait]
pub trait Handler: Send + Sync {
    /// The request type this handler accepts.
    type Request: Send + 'static;

    /// The response type this handler produces.
    type Response: Send + 'static;

    /// Stable identifier for this handler.
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "handler".to_string(),
        })
    }

    /// Route pattern this handler matches.
    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: String::new(),
        })
    }

    /// Execute the handler with the given request and request-scoped context.
    #[allow(clippy::missing_errors_doc)]
    async fn execute(
        &self,
        req: ExecutionRequest<'_, Self::Request>,
    ) -> Result<Self::Response, HandlerError>;

    /// Return `true` if the handler is healthy and able to process requests.
    async fn health_check(
        &self,
        _req: HealthCheckRequest,
    ) -> Result<HealthCheckResponse, HandlerError> {
        Ok(HealthCheckResponse { healthy: true })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler::types::HandlerContext;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;

    struct AlwaysOk;

    #[async_trait]
    impl Handler for AlwaysOk {
        type Request = String;
        type Response = String;

        async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
            Ok(req.req)
        }
    }

    struct AlwaysFail;

    #[async_trait]
    impl Handler for AlwaysFail {
        type Request = String;
        type Response = String;

        async fn execute(
            &self,
            _req: ExecutionRequest<'_, String>,
        ) -> Result<String, HandlerError> {
            Err(HandlerError::ExecutionFailed("fail".into()))
        }
    }

    #[tokio::test]
    async fn test_execute_ok_handler_returns_response_happy() {
        let security = SecurityContext::unauthenticated();
        let bus = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &bus,
            observer: observer.as_ref(),
        };
        assert!(AlwaysOk
            .execute(ExecutionRequest {
                req: "hi".into(),
                ctx: &ctx
            })
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_execute_failing_handler_returns_err_error() {
        let security = SecurityContext::unauthenticated();
        let bus = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &bus,
            observer: observer.as_ref(),
        };
        assert!(AlwaysFail
            .execute(ExecutionRequest {
                req: "hi".into(),
                ctx: &ctx
            })
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_id_default_returns_handler_edge() {
        assert_eq!(AlwaysOk.id(IdRequest).unwrap().id, "handler");
    }

    #[tokio::test]
    async fn test_pattern_default_returns_empty_edge() {
        assert_eq!(AlwaysOk.pattern(PatternRequest).unwrap().pattern, "");
    }

    #[tokio::test]
    async fn test_health_check_default_returns_true_happy() {
        assert!(
            AlwaysOk
                .health_check(HealthCheckRequest)
                .await
                .unwrap()
                .healthy
        );
    }
}
