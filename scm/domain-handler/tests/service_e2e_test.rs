//! End-to-end contract tests for the `Service` trait, exercised through test-double
//! implementations via the crate's public API.

use edge_domain_handler::{HandlerError, Service};
use futures::executor::block_on;

struct EchoService;

#[async_trait::async_trait]
impl Service for EchoService {
    type Request = String;
    type Response = String;

    async fn execute(&self, req: String) -> Result<String, HandlerError> {
        Ok(req)
    }
}

struct FailingService;

#[async_trait::async_trait]
impl Service for FailingService {
    type Request = String;
    type Response = String;

    async fn execute(&self, _req: String) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("boom".into()))
    }
}

/// @covers: Service::execute — successful execution returns the response
#[test]
fn test_execute_ok_service_returns_response_happy() {
    let result = block_on(EchoService.execute("hi".to_string()));
    assert_eq!(result, Ok("hi".to_string()));
}

/// @covers: Service::execute — failing service propagates its error
#[test]
fn test_execute_failing_service_returns_err_error() {
    let result = block_on(FailingService.execute("hi".to_string()));
    assert_eq!(result, Err(HandlerError::ExecutionFailed("boom".into())));
}

/// @covers: Service::execute — empty request round-trips unchanged
#[test]
fn test_execute_empty_request_returns_empty_edge() {
    let result = block_on(EchoService.execute(String::new()));
    assert_eq!(result, Ok(String::new()));
}
