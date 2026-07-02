//! Scenario coverage for `ImageUrl`.

use edge_llm_complete::ImageUrl;

#[test]
fn test_image_url_new_sets_url_happy() {
    let img = ImageUrl::new("https://example.com/a.png");
    assert_eq!(img.url, "https://example.com/a.png");
}

#[test]
fn test_image_url_empty_url_is_valid_error() {
    let img = ImageUrl::new("");
    assert!(img.url.is_empty());
}

#[test]
fn test_image_url_detail_defaults_to_none_edge() {
    let img = ImageUrl::new("https://x.com/b.png");
    assert!(img.detail.is_none());
}

/// @covers: with_detail
#[test]
fn test_image_url_with_detail_sets_detail_happy() {
    let img = ImageUrl::with_detail("https://x.com/c.png", "high");
    assert_eq!(img.detail, Some("high".to_string()));
}
