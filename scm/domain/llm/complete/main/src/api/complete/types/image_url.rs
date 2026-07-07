use serde::{Deserialize, Serialize};

/// URL pointer to an image with an optional detail level hint.
///
/// Orphan-type note: only ever appears nested inside `ContentPart::ImageUrl`, never directly in
/// a trait method signature — same rationale as the note on `ContentPart` itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// Absolute URL to the image resource.
    pub url: String,
    /// Vision detail level (`"low"`, `"high"`, `"auto"`).
    pub detail: Option<String>,
}
