//! Scenario coverage for the `content_flattener_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    ContentFlattener, FlattenRequest, MessageContent, NoopCompleter, CONTENT_FLATTENER_SVC,
};

#[test]
fn test_content_flattener_svc_constant_is_expected_value_happy() {
    assert_eq!(CONTENT_FLATTENER_SVC, "content_flattener");
}

#[test]
fn test_content_flattener_svc_constant_is_nonempty_error() {
    assert!(!CONTENT_FLATTENER_SVC.is_empty());
}

#[test]
fn test_content_flattener_flattens_text_edge() {
    let content = MessageContent::Text("hello".to_string());
    let result = NoopCompleter
        .flatten(FlattenRequest { content: &content })
        .unwrap();
    assert_eq!(result.text, "hello");
}
