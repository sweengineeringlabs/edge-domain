use serde::{Deserialize, Serialize};

/// A single content part (text, image, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentPart {
    /// A text content part.
    Text {
        /// The text content.
        text: String,
    },
    /// An image referenced by URL.
    ImageUrl {
        /// The URL of the image.
        image_url: String,
    },
    /// An inline base64-encoded image.
    ImageBase64 {
        /// The base64-encoded image data.
        data: String,
        /// The media type of the image (e.g. "image/png").
        media_type: String,
    },
}

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
