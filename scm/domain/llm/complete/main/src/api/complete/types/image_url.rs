use serde::{Deserialize, Serialize};

/// URL pointer to an image with an optional detail level hint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// Absolute URL to the image resource.
    pub url: String,
    /// Vision detail level (`"low"`, `"high"`, `"auto"`).
    pub detail: Option<String>,
}

impl ImageUrl {
    /// Construct an image URL with no detail hint.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            detail: None,
        }
    }

    /// Construct an image URL with an explicit detail hint.
    pub fn with_detail(url: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            detail: Some(detail.into()),
        }
    }
}
