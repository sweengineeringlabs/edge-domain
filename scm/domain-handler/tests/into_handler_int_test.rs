//! Integration tests — [`IntoHandler`] extension-trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::NoopCommandBus;
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, IdRequest, IntoHandler, IntoHandlerRequest,
    ObserverContextAdapter, Validator, ValidatorRequest, INTO_HANDLER_SVC,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_service::{NameRequest, NameResponse, Service, ServiceError};
use edge_security_runtime::SecurityContext;
use futures::future::BoxFuture;

fn make_ctx<'a>(
    security: &'a SecurityContext,
    observer: &'a ObserverContextAdapter<'a, dyn edge_domain_observer::ObserverContext>,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: &NoopCommandBus,
        observer,
    }
}

struct EchoService;
impl Service for EchoService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo".to_string(),
        })
    }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

struct FailingService;
impl Service for FailingService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "failing".to_string(),
        })
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Err(ServiceError::Internal("forced failure".into())) })
    }
}

struct EmptyNameService;
impl Service for EmptyNameService {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: String::new(),
        })
    }
    fn execute(&self, _req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(String::new()) })
    }
}

/// @covers: INTO_HANDLER_SVC
#[test]
fn test_into_handler_svc_identifier_is_stable_happy() {
    assert_eq!(INTO_HANDLER_SVC, "into_handler");
}

/// @covers: IntoHandler::into_handler
#[test]
fn test_into_handler_wires_service_name_as_id_happy() {
    let h = EchoService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.id(IdRequest).unwrap().id, "echo");
}

/// @covers: IntoHandler::into_handler
#[test]
fn test_into_handler_empty_name_fails_validation_error() {
    let h = EmptyNameService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert!(h.validate(ValidatorRequest).is_err());
}

/// @covers: IntoHandler::into_handler
#[test]
fn test_into_handler_valid_name_passes_validation_edge() {
    let h = EchoService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    assert_eq!(h.validate(ValidatorRequest), Ok(()));
}

/// @covers: IntoHandler::into_handler
#[tokio::test]
async fn test_into_handler_executes_ok_service_happy() {
    let handler = EchoService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &observer_adapter);
    assert_eq!(
        handler
            .execute(ExecutionRequest {
                req: "hello".into(),
                ctx: &ctx
            })
            .await
            .unwrap(),
        "hello"
    );
}

/// @covers: IntoHandler::into_handler
#[tokio::test]
async fn test_into_handler_propagates_service_error() {
    let handler = FailingService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &observer_adapter);
    let err = handler
        .execute(ExecutionRequest {
            req: "x".into(),
            ctx: &ctx,
        })
        .await
        .unwrap_err();
    assert!(err.to_string().contains("forced failure"));
}

/// @covers: IntoHandler::into_handler
#[tokio::test]
async fn test_into_handler_empty_name_service_executes_edge() {
    let handler = EmptyNameService
        .into_handler(IntoHandlerRequest)
        .unwrap()
        .handler;
    let security = SecurityContext::unauthenticated();
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = make_ctx(&security, &observer_adapter);
    assert_eq!(
        handler
            .execute(ExecutionRequest {
                req: "".into(),
                ctx: &ctx
            })
            .await
            .unwrap(),
        ""
    );
}
