use serde::{Deserialize, Serialize};

/// URL pointer to an image with an optional detail level hint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// Absolute URL to the image resource.
    pub url: String,
    /// Vision detail level (`"low"`, `"high"`, `"auto"`).
    pub detail: Option<String>,
}
