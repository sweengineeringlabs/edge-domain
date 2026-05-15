//! `Service` trait — named domain operation contract.

pub mod service_error;
pub mod service_registry;

pub use service_error::ServiceError;
pub use service_registry::ServiceRegistry;

use async_trait::async_trait;

/// A named, executable domain operation.
///
/// Distinct from [`Handler`](crate::Handler) which is ingress-facing.
/// A `Service` is called from within the domain — by handlers, other
/// services, or background jobs.
///
/// ```rust,ignore
/// #[async_trait]
/// impl Service<CreateOrderRequest, OrderId> for CreateOrderService {
///     fn name(&self) -> &str { "create-order" }
///     async fn execute(&self, req: CreateOrderRequest) -> Result<OrderId, ServiceError> { ... }
/// }
/// ```
#[async_trait]
pub trait Service<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable name used as the lookup key in [`ServiceRegistry`](crate::ServiceRegistry).
    fn name(&self) -> &str;

    /// Execute the service operation.
    async fn execute(&self, req: Request) -> Result<Response, ServiceError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_trait_is_object_safe() {
        fn _assert(_: &dyn Service<String, String>) {}
    }
}
