//! Concrete adapter: wraps a [`Service`](edge_application_service::Service) as a [`Handler`].

use async_trait::async_trait;
use edge_application_service::Service;

use crate::api::ExecutionRequest;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::IdRequest;
use crate::api::IdResponse;
use crate::api::IntoHandler;
use crate::api::IntoHandlerRequest;
use crate::api::IntoHandlerResponse;
use crate::api::ServiceBridge;
use crate::api::ServiceHandler;
use crate::api::Validator;
use crate::api::ValidatorRequest;

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

impl<S: Send + Sync> ServiceHandler for DefaultServiceHandler<S> {}
impl<S: Send + Sync> ServiceBridge for DefaultServiceHandler<S> {}

impl<S: Send + Sync> Validator for DefaultServiceHandler<S> {
    fn validate(&self, _req: ValidatorRequest) -> Result<(), HandlerError> {
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

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.clone(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, S::Request>,
    ) -> Result<S::Response, HandlerError> {
        self.inner
            .execute(req.req)
            .await
            .map_err(HandlerError::from)
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

    #[rustfmt::skip]
    fn into_handler(self, _req: IntoHandlerRequest) -> Result<IntoHandlerResponse<impl Handler<Request = Self::Request, Response = Self::Response> + ServiceHandler>, HandlerError> {
        let id = self
            .name(edge_application_service::NameRequest)
            .map_err(HandlerError::from)?
            .name;
        Ok(IntoHandlerResponse {
            handler: DefaultServiceHandler::new(id, self),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_application_service::{NoopRequest, NoopResponse, ServiceError};
    use futures::future::BoxFuture;

    struct DefaultServiceHandlerStub;

    impl Service for DefaultServiceHandlerStub {
        type Request = NoopRequest;
        type Response = NoopResponse;
        fn name(
            &self,
            _req: edge_application_service::NameRequest,
        ) -> Result<edge_application_service::NameResponse, ServiceError> {
            Ok(edge_application_service::NameResponse {
                name: "stub".to_string(),
            })
        }
        fn execute(&self, _: NoopRequest) -> BoxFuture<'_, Result<NoopResponse, ServiceError>> {
            Box::pin(async { Ok(NoopResponse) })
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
        assert!(h.validate(ValidatorRequest).is_err());
    }

    #[test]
    fn test_validate_nonempty_id_returns_ok_edge() {
        let h = DefaultServiceHandler::new("svc".to_string(), DefaultServiceHandlerStub);
        assert_eq!(h.validate(ValidatorRequest), Ok(()));
    }
}
