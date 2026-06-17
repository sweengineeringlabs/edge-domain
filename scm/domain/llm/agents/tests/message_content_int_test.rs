#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `MessageContent` value type.

use edge_llm_agent::{ContentPart, MessageContent};

#[test]
fn test_message_content_text_constructor() {
    assert_eq!(
        MessageContent::text("hi"),
        MessageContent::Text("hi".to_string())
    );
}

#[test]
fn test_message_content_parts_constructor() {
    let content = MessageContent::parts(vec![ContentPart::text("a")]);
    match content {
        MessageContent::Parts(parts) => assert_eq!(parts.len(), 1),
        MessageContent::Text(_) => panic!("expected Parts variant"),
    }
}

#[test]
fn test_message_content_from_str() {
    let content: MessageContent = "literal".into();
    assert_eq!(content, MessageContent::Text("literal".to_string()));
}
