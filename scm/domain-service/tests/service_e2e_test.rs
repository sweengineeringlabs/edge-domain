//! End-to-end contract tests for the `Service` trait, exercised through a
//! test-double implementation via the crate's public API.

use edge_application_service::{
    NameRequest, NameResponse, NoopRequest, NoopResponse, Service, ServiceError,
};
use futures::executor::block_on;
use futures::future::BoxFuture;
use std::sync::Arc;

struct TestService;

impl Service for TestService {
    type Request = NoopRequest;
    type Response = NoopResponse;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "test".to_string(),
        })
    }

    fn execute(&self, _req: NoopRequest) -> BoxFuture<'_, Result<NoopResponse, ServiceError>> {
        Box::pin(async move { Ok(NoopResponse) })
    }
}

struct FailingService;

impl Service for FailingService {
    type Request = NoopRequest;
    type Response = NoopResponse;

    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Err(ServiceError::NotFound("failing-service".to_string()))
    }

    fn execute(&self, _req: NoopRequest) -> BoxFuture<'_, Result<NoopResponse, ServiceError>> {
        Box::pin(async move { Err(ServiceError::NotFound("failing-service".to_string())) })
    }
}

/// @covers: Service::name
#[test]
fn test_name_returns_ok_happy() {
    let svc = TestService;
    let result = svc.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "test"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}

/// @covers: Service::name
#[test]
fn test_name_failing_service_returns_err_error() {
    let svc = FailingService;
    let result = svc.name(NameRequest);
    assert!(result.is_err());
}

/// @covers: Service::name
#[test]
fn test_name_consistent_edge() {
    let svc = TestService;
    let r1 = svc.name(NameRequest);
    let r2 = svc.name(NameRequest);
    assert_eq!(r1, r2);
}

/// @covers: Service::execute
#[test]
fn test_execute_returns_ok_happy() {
    let svc = TestService;
    let result = block_on(svc.execute(NoopRequest));
    assert_eq!(result, Ok(NoopResponse));
}

/// @covers: Service::execute
#[test]
fn test_execute_failing_returns_err_error() {
    let svc = FailingService;
    let result = block_on(svc.execute(NoopRequest));
    assert!(result.is_err());
}

/// @covers: Service::execute
#[test]
fn test_execute_idempotent_edge() {
    let svc = TestService;
    for _ in 0..3 {
        assert_eq!(block_on(svc.execute(NoopRequest)), Ok(NoopResponse));
    }
}

/// @covers: Service
#[test]
fn test_service_as_dyn_trait_edge() {
    let svc: Arc<dyn Service<Request = NoopRequest, Response = NoopResponse>> = Arc::new(TestService);
    let result = svc.name(NameRequest);
    match result {
        Ok(response) => assert_eq!(response.name, "test"),
        Err(err) => panic!("expected Ok, got Err: {err:?}"),
    }
}
