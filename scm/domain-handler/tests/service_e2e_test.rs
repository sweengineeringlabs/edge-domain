//! End-to-end contract tests for the `Service` trait, exercised through test-double
//! implementations via the crate's public API.

use edge_application_handler::{HandlerError, Service};
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoService;

#[async_trait::async_trait]
impl Service for EchoService {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(&self, req: TextPayload) -> Result<TextPayload, HandlerError> {
        Ok(req)
    }
}

struct FailingService;

#[async_trait::async_trait]
impl Service for FailingService {
    type Request = TextPayload;
    type Response = TextPayload;

    async fn execute(&self, _req: TextPayload) -> Result<TextPayload, HandlerError> {
        Err(HandlerError::ExecutionFailed("boom".into()))
    }
}

/// @covers: Service::execute — successful execution returns the response
#[test]
fn test_execute_ok_service_returns_response_happy() {
    let result = block_on(EchoService.execute(TextPayload("hi".to_string())));
    assert_eq!(result, Ok(TextPayload("hi".to_string())));
}

/// @covers: Service::execute — failing service propagates its error
#[test]
fn test_execute_failing_service_returns_err_error() {
    let result = block_on(FailingService.execute(TextPayload("hi".to_string())));
    assert_eq!(result, Err(HandlerError::ExecutionFailed("boom".into())));
}

/// @covers: Service::execute — empty request round-trips unchanged
#[test]
fn test_execute_empty_request_returns_empty_edge() {
    let result = block_on(EchoService.execute(TextPayload(String::new())));
    assert_eq!(result, Ok(TextPayload(String::new())));
}
