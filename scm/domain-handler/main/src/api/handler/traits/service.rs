//! `Service` trait — local decoupling boundary for a bridged domain service.

use async_trait::async_trait;

use crate::api::handler::errors::HandlerError;

/// An async unit of domain logic looked up from a [`ServiceRegistry`](super::ServiceRegistry).
///
/// Declared locally so `api/` never references `edge_domain_service::Service` directly in a
/// type position (SEA `no_foreign_type`). Any real `Service` implementor is reachable through
/// this boundary via the adapter constructed in `core/`.
#[async_trait]
pub trait Service: Send + Sync {
    /// The request type this service accepts.
    type Request: Send + 'static;

    /// The response type this service produces.
    type Response: Send + 'static;

    /// Execute the service with the given request.
    #[allow(clippy::missing_errors_doc)]
    async fn execute(&self, req: Self::Request) -> Result<Self::Response, HandlerError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysOk;

    #[async_trait]
    impl Service for AlwaysOk {
        type Request = String;
        type Response = String;

        async fn execute(&self, req: String) -> Result<String, HandlerError> {
            Ok(req)
        }
    }

    struct AlwaysFail;

    #[async_trait]
    impl Service for AlwaysFail {
        type Request = String;
        type Response = String;

        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Err(HandlerError::ExecutionFailed("fail".into()))
        }
    }

    #[tokio::test]
    async fn test_execute_ok_service_returns_response_happy() {
        assert_eq!(AlwaysOk.execute("hi".into()).await, Ok("hi".to_string()));
    }

    #[tokio::test]
    async fn test_execute_failing_service_returns_err_error() {
        assert!(AlwaysFail.execute("hi".into()).await.is_err());
    }

    #[tokio::test]
    async fn test_execute_empty_request_returns_empty_edge() {
        assert_eq!(AlwaysOk.execute(String::new()).await, Ok(String::new()));
    }
}
