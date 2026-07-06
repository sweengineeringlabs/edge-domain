//! Constructors for [`ImageUrl`].

use crate::api::ImageUrl;

impl ImageUrl {
    /// Construct an image URL with no detail hint.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: Self::normalized_url(url.into()),
            detail: None,
        }
    }

    /// Construct an image URL with an explicit detail hint.
    pub fn with_detail(url: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            url: Self::normalized_url(url.into()),
            detail: Some(detail.into()),
        }
    }

    /// Strip leading/trailing whitespace from a URL string.
    fn normalized_url(url: String) -> String {
        url.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_has_no_detail() {
        assert!(ImageUrl::new("http://x").detail.is_none());
    }

    /// @covers: with_detail
    #[test]
    fn test_with_detail_sets_detail() {
        let img = ImageUrl::with_detail("http://x", "high");
        assert_eq!(img.detail, Some("high".to_string()));
    }

    /// @covers: normalized_url
    #[test]
    fn test_normalized_url_strips_whitespace() {
        assert_eq!(
            ImageUrl::normalized_url("  http://x  ".to_string()),
            "http://x"
        );
    }
}
