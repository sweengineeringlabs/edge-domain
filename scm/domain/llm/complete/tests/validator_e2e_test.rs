//! Scenario coverage for the `Validator` trait.

use edge_llm_complete::{CompleteError, CompletionRequest, Message, Validator};

struct MinimalValidator;

impl Validator for MinimalValidator {
    fn validate(&self, request: &CompletionRequest) -> Result<(), CompleteError> {
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
    assert!(MinimalValidator.validate(&req).is_ok());
}

#[test]
fn test_validate_empty_model_returns_invalid_request_error() {
    let req = CompletionRequest::new("", vec![Message::user("hi")]);
    let err = MinimalValidator.validate(&req).unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}

#[test]
fn test_validate_empty_messages_returns_invalid_request_edge() {
    let req = CompletionRequest::new("gpt-4", vec![]);
    let err = MinimalValidator.validate(&req).unwrap_err();
    assert!(matches!(err, CompleteError::InvalidRequest(_)));
}
