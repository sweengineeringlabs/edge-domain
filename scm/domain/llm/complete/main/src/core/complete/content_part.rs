//! Constructors for [`ContentPart`].

use crate::api::{ContentPart, ImageUrl};

impl ContentPart {
    /// Construct a plain-text part.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Construct an image URL part.
    pub fn image_url(image_url: ImageUrl) -> Self {
        Self::ImageUrl {
            image_url: Box::new(image_url),
        }
    }

    /// Construct a base64-image part.
    pub fn image_base64(data: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::ImageBase64 {
            data: data.into(),
            media_type: Self::normalized_media_type(media_type.into()),
        }
    }

    /// Trim and lowercase a MIME media type string.
    fn normalized_media_type(media_type: String) -> String {
        media_type.trim().to_lowercase()
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

    /// @covers: image_url
    #[test]
    fn test_image_url_constructs_image_url_variant() {
        let part = ContentPart::image_url(ImageUrl::new("http://x"));
        assert!(matches!(part, ContentPart::ImageUrl { .. }));
    }

    /// @covers: image_base64
    #[test]
    fn test_image_base64_constructs_image_base64_variant() {
        let part = ContentPart::image_base64("YQ==", "image/png");
        assert!(matches!(part, ContentPart::ImageBase64 { .. }));
    }

    /// @covers: normalized_media_type
    #[test]
    fn test_normalized_media_type_trims_and_lowercases() {
        assert_eq!(
            ContentPart::normalized_media_type("  IMAGE/PNG  ".to_string()),
            "image/png"
        );
    }
}
