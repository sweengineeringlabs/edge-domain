//! Scenario coverage for the `Validator` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{CompleteError, CompletionRequest, Message, ValidationRequest, Validator};

struct MinimalValidator;

impl Validator for MinimalValidator {
    fn validate(&self, req: ValidationRequest<'_>) -> Result<(), CompleteError> {
        let request = req.request;
        if request.model.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "model is required".to_string(),
            ));
        }
        if request.messages.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "messages must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[test]
fn test_validate_valid_request_returns_ok_happy() {
    let req = CompletionRequest::new("gpt-4", vec![Message::user("hi")]);
    let result = MinimalValidator.validate(ValidationRequest { request: &req });
    assert!(matches!(result, Ok(())));
}

#[test]
fn test_validate_empty_model_returns_invalid_request_error() {
    let req = CompletionRequest::new("", vec![Message::user("hi")]);
    let err = MinimalValidator
        .validate(ValidationRequest { request: &req })
        .unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}

#[test]
fn test_validate_empty_messages_returns_invalid_request_edge() {
    let req = CompletionRequest::new("gpt-4", vec![]);
    let err = MinimalValidator
        .validate(ValidationRequest { request: &req })
        .unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}
