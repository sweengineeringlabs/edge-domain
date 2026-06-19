use edge_llm_complete::{CompletionResponse, FinishReason};

#[test]
fn test_text_constructor_sets_content_and_stop() {
    let resp = CompletionResponse::text("r-1", "gpt-4", "hello");
    assert_eq!(resp.content, Some("hello".to_string()));
    assert_eq!(resp.finish_reason, FinishReason::Stop);
}

#[test]
fn test_default_has_no_tool_calls() {
    assert!(CompletionResponse::default().tool_calls.is_empty());
}

#[test]
fn test_roundtrip_serialization() {
    let resp = CompletionResponse::text("x", "m", "hi");
    let json = serde_json::to_string(&resp).unwrap();
    let back: CompletionResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(resp, back);
}
