use serde::{Deserialize, Serialize};

use crate::api::complete::types::ImageUrl;

/// A single part of a multi-modal message body.
///
/// Orphan-type note: only ever appears nested inside `MessageContent` (itself referenced by
/// `ContentFlattener::flatten`'s request type) — never directly as a trait method parameter or
/// return type. `no_orphan_types` only checks direct references, not nested ones, so this flags
/// as an orphan. There is no interface to expose here: `ContentPart` is a plain data enum, not a
/// trait-object wrapper, so inventing a trait solely to reference it would be ceremony with no
/// real polymorphism behind it.
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
