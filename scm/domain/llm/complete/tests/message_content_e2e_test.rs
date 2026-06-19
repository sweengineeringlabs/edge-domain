//! Scenario coverage for `MessageContent`.

use edge_llm_complete::{ContentPart, MessageContent};

#[test]
fn test_message_content_text_variant_holds_string_happy() {
    let content = MessageContent::Text("hello".to_string());
    assert!(matches!(content, MessageContent::Text(s) if s == "hello"));
}

#[test]
fn test_message_content_parts_variant_is_empty_initially_error() {
    let content = MessageContent::Parts(vec![]);
    assert!(matches!(content, MessageContent::Parts(p) if p.is_empty()));
}

#[test]
fn test_message_content_parts_variant_holds_multiple_parts_edge() {
    let parts = vec![
        ContentPart::text("a"),
        ContentPart::text("b"),
    ];
    let content = MessageContent::Parts(parts);
    assert!(matches!(&content, MessageContent::Parts(p) if p.len() == 2));
}
