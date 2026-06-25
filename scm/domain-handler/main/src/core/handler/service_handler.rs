//! Concrete adapter: wraps a [`Service`](edge_domain_service::Service) as a [`Handler`].

use async_trait::async_trait;
use edge_domain_service::Service;

use crate::api::Handler;
use crate::api::HandlerContext;
use crate::api::HandlerError;
use crate::api::IntoHandler;
use crate::api::ServiceHandler as ServiceHandlerTrait;
use crate::api::ServiceBridge;
use crate::api::Validator;

/// Wraps a [`Service`] impl and exposes it as a [`Handler`].
///
/// `pub(crate)` — consumers receive `impl Handler + ServiceHandler` via
/// [`IntoHandler::into_handler`], never this concrete type.
pub(crate) struct DefaultServiceHandler<S> {
    id: String,
    inner: S,
}

impl<S> DefaultServiceHandler<S> {
    pub(crate) fn new(id: String, inner: S) -> Self {
        Self { id, inner }
    }
}

impl<S: Send + Sync> ServiceHandlerTrait for DefaultServiceHandler<S> {}
impl<S: Send + Sync> ServiceBridge for DefaultServiceHandler<S> {}

impl<S: Send + Sync> Validator for DefaultServiceHandler<S> {
    fn validate(&self) -> Result<(), HandlerError> {
        if self.id.is_empty() {
            Err(HandlerError::InvalidRequest(
                "service name cannot be empty".into(),
            ))
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl<S> Handler for DefaultServiceHandler<S>
where
    S: Service + Send + Sync,
    S::Request: Send + 'static,
    S::Response: Send + 'static,
{
    type Request = S::Request;
    type Response = S::Response;

    fn id(&self) -> &str {
        &self.id
    }

    async fn execute(
        &self,
        req: S::Request,
        _ctx: HandlerContext<'_>,
    ) -> Result<S::Response, HandlerError> {
        self.inner.execute(req).await.map_err(HandlerError::from)
    }
}

impl<S> IntoHandler for S
where
    S: Service + Send + Sync,
    S::Request: Send + 'static,
    S::Response: Send + 'static,
{
    type Request = S::Request;
    type Response = S::Response;

    fn into_handler(
        self,
    ) -> impl Handler<Request = Self::Request, Response = Self::Response> + ServiceHandlerTrait
    {
        let id = self.name().to_string();
        DefaultServiceHandler::new(id, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_service::ServiceError;
    use futures::future::BoxFuture;

    struct DefaultServiceHandlerStub;

    impl Service for DefaultServiceHandlerStub {
        type Request = ();
        type Response = ();
        fn name(&self) -> &str { "stub" }
        fn execute(&self, _: ()) -> BoxFuture<'_, Result<(), ServiceError>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn test_new_stores_id_happy() {
        let h = DefaultServiceHandler::new("stub.service".to_string(), DefaultServiceHandlerStub);
        assert_eq!(h.id, "stub.service");
    }

    #[test]
    fn test_validate_empty_id_returns_error_error() {
        let h = DefaultServiceHandler::new(String::new(), DefaultServiceHandlerStub);
        assert!(h.validate().is_err());
    }

    #[test]
    fn test_validate_nonempty_id_returns_ok_edge() {
        let h = DefaultServiceHandler::new("svc".to_string(), DefaultServiceHandlerStub);
        assert_eq!(h.validate(), Ok(()));
    }
}
