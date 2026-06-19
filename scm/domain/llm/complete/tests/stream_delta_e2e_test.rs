//! Scenario coverage for `StreamDelta`.

use edge_llm_complete::StreamDelta;

#[test]
fn test_stream_delta_text_sets_content_happy() {
    let delta = StreamDelta::text("hello");
    assert_eq!(delta.content, Some("hello".to_string()));
}

#[test]
fn test_stream_delta_empty_has_no_content_error() {
    let delta = StreamDelta::empty();
    assert!(delta.content.is_none());
    assert!(delta.tool_calls.is_none());
}

#[test]
fn test_stream_delta_text_with_empty_string_is_valid_edge() {
    let delta = StreamDelta::text("");
    assert_eq!(delta.content, Some(String::new()));
}
