//! Scenario coverage for `ContentPart`.

use edge_llm_complete::{ContentPart, ImageUrl};

/// @covers: text
#[test]
fn test_content_part_text_variant_sets_text_happy() {
    let part = ContentPart::text("hello");
    assert!(matches!(part, ContentPart::Text { text } if text == "hello"));
}

/// @covers: image_url
#[test]
fn test_content_part_image_url_variant_wraps_url_error() {
    let img = ImageUrl::new("https://example.com/img.png");
    let part = ContentPart::image_url(img.clone());
    assert!(matches!(&part, ContentPart::ImageUrl { image_url } if image_url.url == img.url));
}

/// @covers: image_base64
#[test]
fn test_content_part_base64_sets_media_type_edge() {
    let part = ContentPart::image_base64("abc123", "image/png");
    assert!(
        matches!(&part, ContentPart::ImageBase64 { media_type, .. } if media_type == "image/png")
    );
}
