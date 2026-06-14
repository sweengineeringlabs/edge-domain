//! `Service` trait — named domain operation contract.

use futures::future::BoxFuture;

use crate::api::service::ServiceError;

/// A named, executable domain operation.
///
/// Distinct from [`Handler`](crate::Handler) which is ingress-facing.
/// A `Service` is called from within the domain — by handlers, other
/// services, or background jobs.
///
/// ```rust,ignore
/// impl Service for CreateOrderService {
///     type Request = CreateOrderRequest;
///     type Response = OrderId;
///
///     fn name(&self) -> &str { "create-order" }
///     fn execute(&self, req: CreateOrderRequest) -> BoxFuture<'_, Result<OrderId, ServiceError>> {
///         Box::pin(async move { ... })
///     }
/// }
/// ```
pub trait Service: Send + Sync {
    /// The request type accepted by this service.
    type Request: Send + 'static;
    /// The response type produced by this service.
    type Response: Send + 'static;

    /// Stable name used as the lookup key in [`ServiceRegistry`](crate::ServiceRegistry).
    fn name(&self) -> &str {
        "service"
    }

    /// Execute the service operation.
    fn execute(&self, req: Self::Request) -> BoxFuture<'_, Result<Self::Response, ServiceError>>;
}
