use serde::{Deserialize, Serialize};

use crate::api::complete::types::ImageUrl;

/// A single part of a multi-modal message body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    /// Plain text fragment.
    Text {
        /// Text content.
        text: String,
    },
    /// Image referenced by URL.
    ImageUrl {
        /// Image location and detail hint.
        image_url: Box<ImageUrl>,
    },
    /// Base64-encoded image data.
    ImageBase64 {
        /// Base64-encoded bytes.
        data: String,
        /// MIME type (e.g. `"image/png"`).
        media_type: String,
    },
}

impl ContentPart {
    /// Construct a plain-text part.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Construct an image URL part.
    pub fn image_url(image_url: ImageUrl) -> Self {
        Self::ImageUrl { image_url: Box::new(image_url) }
    }

    /// Construct a base64-image part.
    pub fn image_base64(data: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::ImageBase64 { data: data.into(), media_type: media_type.into() }
    }
}
