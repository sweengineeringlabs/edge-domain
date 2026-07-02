//! Constructors for [`ContentPart`].

use crate::api::ContentPart;

impl ContentPart {
    /// Creates a text content part.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Creates an image content part referenced by URL.
    pub fn image_url(url: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: url.into(),
        }
    }

    /// Creates an inline base64-encoded image content part.
    pub fn image_base64(data: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::ImageBase64 {
            data: data.into(),
            media_type: media_type.into(),
        }
    }
}

impl Default for ContentPart {
    fn default() -> Self {
        Self::text("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_text_constructs_text_variant() {
        assert!(matches!(ContentPart::text("hi"), ContentPart::Text { .. }));
    }

    /// @covers: default
    #[test]
    fn test_default_is_empty_text_variant() {
        assert!(matches!(ContentPart::default(), ContentPart::Text { text } if text.is_empty()));
    }

    /// @covers: image_url
    #[test]
    fn test_image_url_constructs_image_url_variant() {
        assert!(matches!(
            ContentPart::image_url("http://x"),
            ContentPart::ImageUrl { .. }
        ));
    }

    /// @covers: image_base64
    #[test]
    fn test_image_base64_constructs_image_base64_variant() {
        assert!(matches!(
            ContentPart::image_base64("YQ==", "image/png"),
            ContentPart::ImageBase64 { .. }
        ));
    }
}
