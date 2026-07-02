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
