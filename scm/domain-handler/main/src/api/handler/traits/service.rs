//! `Service` trait — local decoupling boundary for a bridged domain service.

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;

/// An async unit of domain logic looked up from a [`ServiceRegistry`](super::ServiceRegistry).
///
/// Declared locally so `api/` never references `edge_application_service::Service` directly in a
/// type position (SEA `no_foreign_type`). Any real `Service` implementor is reachable through
/// this boundary via the adapter constructed in `core/`.
#[async_trait]
pub trait Service: Send + Sync {
    /// The request type this service accepts.
    type Request: edge_application_base::Request;

    /// The response type this service produces.
    type Response: edge_application_base::Response;

    /// Execute the service with the given request.
    #[allow(clippy::missing_errors_doc)]
    async fn execute(&self, req: Self::Request) -> Result<Self::Response, HandlerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TextPayload(String);

    impl edge_application_base::Request for TextPayload {}
    impl edge_application_base::Response for TextPayload {}

    struct AlwaysOk;

    #[async_trait]
    impl Service for AlwaysOk {
        type Request = TextPayload;
        type Response = TextPayload;

        async fn execute(&self, req: TextPayload) -> Result<TextPayload, HandlerError> {
            Ok(req)
        }
    }

    struct AlwaysFail;

    #[async_trait]
    impl Service for AlwaysFail {
        type Request = TextPayload;
        type Response = TextPayload;

        async fn execute(&self, _req: TextPayload) -> Result<TextPayload, HandlerError> {
            Err(HandlerError::ExecutionFailed("fail".into()))
        }
    }

    #[tokio::test]
    async fn test_execute_ok_service_returns_response_happy() {
        assert_eq!(
            AlwaysOk.execute(TextPayload("hi".into())).await,
            Ok(TextPayload("hi".to_string()))
        );
    }

    #[tokio::test]
    async fn test_execute_failing_service_returns_err_error() {
        assert!(AlwaysFail.execute(TextPayload("hi".into())).await.is_err());
    }

    #[tokio::test]
    async fn test_execute_empty_request_returns_empty_edge() {
        assert_eq!(
            AlwaysOk.execute(TextPayload(String::new())).await,
            Ok(TextPayload(String::new()))
        );
    }
}
