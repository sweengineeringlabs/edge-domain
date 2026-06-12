//! `Service` trait — a named domain operation that processes a request and returns a response.

use futures::future::BoxFuture;

use crate::api::service::ServiceError;

/// A named domain operation that processes a typed request and produces a typed response.
///
/// Services are the primary abstraction for application-layer logic. They are
/// identified by name and invoked via the [`ServiceRegistry`](super::service_registry::ServiceRegistry).
pub trait Service<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable name identifying this service.
    fn name(&self) -> &str {
        "service"
    }

    /// Execute the service with the given request.
    fn execute(&self, req: Request) -> BoxFuture<'_, Result<Response, ServiceError>>;
}
