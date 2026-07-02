use serde::{Deserialize, Serialize};

use crate::api::complete::types::ContentPart;

/// The body of a [`Message`](crate::api::complete::types::Message) — either plain text or a mixed-media list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text body.
    #[default]
    Empty,
    /// Plain text body.
    Text(String),
    /// Multi-modal parts (text, images, …).
    Parts(Vec<ContentPart>),
}
