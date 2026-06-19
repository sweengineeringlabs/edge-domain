//! Scenario coverage for the `ContentFlattener` trait.

use edge_llm_complete::{ContentFlattener, ContentPart, ImageUrl, MessageContent};

struct PlainFlattener;

impl ContentFlattener for PlainFlattener {
    fn flatten(&self, content: &MessageContent) -> String {
        match content {
            MessageContent::Text(t) => t.clone(),
            MessageContent::Parts(parts) => parts
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(""),
            MessageContent::Empty => String::new(),
        }
    }
}

#[test]
fn test_flatten_text_content_returns_string_happy() {
    let content = MessageContent::Text("hello world".to_string());
    assert_eq!(PlainFlattener.flatten(&content), "hello world");
}

#[test]
fn test_flatten_empty_content_returns_empty_string_error() {
    assert_eq!(PlainFlattener.flatten(&MessageContent::Empty), "");
}

#[test]
fn test_flatten_parts_concatenates_text_parts_edge() {
    let parts = vec![
        ContentPart::text("foo"),
        ContentPart::image_url(ImageUrl::new("https://x.com/img.png")),
        ContentPart::text("bar"),
    ];
    let content = MessageContent::Parts(parts);
    assert_eq!(PlainFlattener.flatten(&content), "foobar");
}
