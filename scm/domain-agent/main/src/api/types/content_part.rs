use serde::{Deserialize, Serialize};

/// A single content part (text, image, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: String },
    ImageBase64 { data: String, media_type: String },
}

impl ContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    pub fn image_url(url: impl Into<String>) -> Self {
        Self::ImageUrl { image_url: url.into() }
    }

    pub fn image_base64(data: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::ImageBase64 {
            data: data.into(),
            media_type: media_type.into(),
        }
    }
}
