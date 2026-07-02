//! Scenario coverage for the `Processor` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_llm_complete::{
    CompleteError, CompleteRequest, Completer, CompletionRequest, CompletionResponse,
    EchoCompleter, Message, ProcessingRequest, Processor,
};
use futures::executor::block_on;

struct PassthroughProcessor;

#[async_trait]
impl Processor for PassthroughProcessor {
    async fn process(
        &self,
        req: ProcessingRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        EchoCompleter
            .complete(CompleteRequest {
                request: req.request,
            })
            .await
    }
}

struct RejectingProcessor;

#[async_trait]
impl Processor for RejectingProcessor {
    async fn process(
        &self,
        _req: ProcessingRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        Err(CompleteError::InvalidRequest("rejected".to_string()))
    }
}

#[test]
fn test_process_valid_request_returns_response_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("go")]);
    let resp = block_on(PassthroughProcessor.process(ProcessingRequest { request: &req })).unwrap();
    assert_eq!(resp.content, Some("go".to_string()));
}

#[test]
fn test_process_rejecting_processor_returns_error_error() {
    let req = CompletionRequest::new("echo", vec![]);
    let err =
        block_on(RejectingProcessor.process(ProcessingRequest { request: &req })).unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}

#[test]
fn test_process_empty_messages_completes_without_panic_edge() {
    let req = CompletionRequest::new("echo", vec![]);
    let resp = block_on(PassthroughProcessor.process(ProcessingRequest { request: &req })).unwrap();
    assert_eq!(resp.content, Some(String::new()));
}
