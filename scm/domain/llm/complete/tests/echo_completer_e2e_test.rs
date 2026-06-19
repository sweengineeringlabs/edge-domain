//! Scenario coverage for `EchoCompleter`.

use edge_llm_complete::{
    CompleteError, Completer, CompletionRequest, EchoCompleter, Message, Processor, Validator,
};
use futures::executor::block_on;

#[test]
fn test_echo_completer_echoes_last_user_message_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("hello")]);
    let resp = block_on(EchoCompleter.complete(&req)).unwrap();
    assert_eq!(resp.content, Some("hello".to_string()));
}

#[test]
fn test_echo_completer_model_info_unknown_returns_error_error() {
    let err = block_on(EchoCompleter.model_info("unknown")).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

#[test]
fn test_echo_completer_supported_models_contains_echo_edge() {
    assert!(EchoCompleter.supported_models().contains(&"echo".to_string()));
}

#[test]
fn test_echo_completer_processor_delegates_to_complete_happy() {
    let req = CompletionRequest::new("echo", vec![Message::user("via processor")]);
    let resp = block_on(EchoCompleter.process(&req)).unwrap();
    assert_eq!(resp.content, Some("via processor".to_string()));
}

#[test]
fn test_echo_completer_validator_rejects_empty_model_error() {
    let req = CompletionRequest::new("", vec![]);
    assert!(EchoCompleter.validate(&req).is_err());
}

#[test]
fn test_echo_completer_validator_accepts_valid_request_edge() {
    let req = CompletionRequest::new("echo", vec![Message::user("ok")]);
    assert!(EchoCompleter.validate(&req).is_ok());
}
