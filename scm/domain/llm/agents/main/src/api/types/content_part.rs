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
