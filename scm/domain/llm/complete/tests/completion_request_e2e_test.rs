use edge_llm_complete::{CompletionRequest, Message};

#[test]
fn test_new_sets_model_and_messages() {
    let req = CompletionRequest::new("gpt-4", vec![Message::user("hi")]);
    assert_eq!(req.model, "gpt-4");
    assert_eq!(req.messages.len(), 1);
}

#[test]
fn test_default_has_all_options_none() {
    let req = CompletionRequest::default();
    assert!(req.temperature.is_none());
    assert!(req.max_tokens.is_none());
    assert!(req.tools.is_none());
}

#[test]
fn test_roundtrip_serialization() {
    let req = CompletionRequest::new("m", vec![Message::user("hello")]);
    let json = serde_json::to_string(&req).unwrap();
    let back: CompletionRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(req, back);
}
