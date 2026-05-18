//! `Service` trait — named domain operation contract.

use futures::future::BoxFuture;

use super::service_error::ServiceError;

/// A named, executable domain operation.
///
/// Distinct from [`Handler`](crate::Handler) which is ingress-facing.
/// A `Service` is called from within the domain — by handlers, other
/// services, or background jobs.
///
/// ```rust,ignore
/// impl Service<CreateOrderRequest, OrderId> for CreateOrderService {
///     fn name(&self) -> &str { "create-order" }
///     fn execute(&self, req: CreateOrderRequest) -> BoxFuture<'_, Result<OrderId, ServiceError>> {
///         Box::pin(async move { ... })
///     }
/// }
/// ```
pub trait Service<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Stable name used as the lookup key in [`ServiceRegistry`](crate::ServiceRegistry).
    fn name(&self) -> &str;

    /// Execute the service operation.
    fn execute(&self, req: Request) -> BoxFuture<'_, Result<Response, ServiceError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_trait_is_object_safe() {
        fn _assert(_: &dyn Service<String, String>) {}
    }

    struct EchoService;
    impl Service<String, String> for EchoService {
        fn name(&self) -> &str {
            "echo"
        }
        fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
            Box::pin(async move { Ok(req) })
        }
    }

    #[tokio::test]
    async fn test_execute_returns_input() {
        assert_eq!(EchoService.execute("hi".into()).await.unwrap(), "hi");
    }
}
