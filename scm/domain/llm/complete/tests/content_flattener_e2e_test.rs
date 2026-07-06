//! Scenario coverage for the `ContentFlattener` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    CompleteError, ContentFlattener, ContentPart, FlattenRequest, FlattenResponse, ImageUrl,
    MessageContent,
};

struct PlainFlattener;

impl ContentFlattener for PlainFlattener {
    fn flatten(&self, req: FlattenRequest<'_>) -> Result<FlattenResponse, CompleteError> {
        let text = match req.content {
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
        };
        Ok(FlattenResponse { text })
    }
}

#[test]
fn test_flatten_text_content_returns_string_happy() {
    let content = MessageContent::Text("hello world".to_string());
    let result = PlainFlattener
        .flatten(FlattenRequest { content: &content })
        .unwrap();
    assert_eq!(result.text, "hello world");
}

#[test]
fn test_flatten_empty_content_returns_empty_string_error() {
    let result = PlainFlattener
        .flatten(FlattenRequest {
            content: &MessageContent::Empty,
        })
        .unwrap();
    assert_eq!(result.text, "");
}

#[test]
fn test_flatten_parts_concatenates_text_parts_edge() {
    let parts = vec![
        ContentPart::text("foo"),
        ContentPart::image_url(ImageUrl::new("https://x.com/img.png")),
        ContentPart::text("bar"),
    ];
    let content = MessageContent::Parts(parts);
    let result = PlainFlattener
        .flatten(FlattenRequest { content: &content })
        .unwrap();
    assert_eq!(result.text, "foobar");
}
