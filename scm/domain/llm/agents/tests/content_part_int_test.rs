#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ContentPart` value type.

use edge_llm_agent::ContentPart;

#[test]
fn test_content_part_text_constructor() {
    match ContentPart::text("hi") {
        ContentPart::Text { text } => assert_eq!(text, "hi"),
        _ => panic!("expected Text variant"),
    }
}

#[test]
fn test_content_part_image_url_constructor() {
    match ContentPart::image_url("http://x/y.png") {
        ContentPart::ImageUrl { image_url } => assert_eq!(image_url, "http://x/y.png"),
        _ => panic!("expected ImageUrl variant"),
    }
}

#[test]
fn test_content_part_image_base64_carries_media_type() {
    match ContentPart::image_base64("ZGF0YQ==", "image/png") {
        ContentPart::ImageBase64 { data, media_type } => {
            assert_eq!(data, "ZGF0YQ==");
            assert_eq!(media_type, "image/png");
        }
        _ => panic!("expected ImageBase64 variant"),
    }
}
