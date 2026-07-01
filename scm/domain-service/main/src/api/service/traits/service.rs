//! `Service` trait — a named domain operation that processes a request and returns a response.

use futures::future::BoxFuture;

use crate::api::service::{ServiceError, NameRequest, NameResponse};

/// A named domain operation that processes a typed request and produces a typed response.
///
/// Services are the primary abstraction for application-layer logic. They are
/// identified by name and invoked via the [`ServiceRegistry`](super::service_registry::ServiceRegistry).
pub trait Service: Send + Sync {
    /// The request type this service accepts.
    type Request: Send + 'static;
    /// The response type this service produces.
    type Response: Send + 'static;

    /// Query the stable name identifying this service.
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse { name: "service".to_string() })
    }

    /// Execute the service with the given request.
    fn execute(&self, req: Self::Request) -> BoxFuture<'_, Result<Self::Response, ServiceError>>;
}
