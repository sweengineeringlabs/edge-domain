//! `ContentFlattener` — flattens structured [`MessageContent`] to plain text.

use crate::api::complete::types::MessageContent;

/// Reduces a [`MessageContent`] to a plain-text string for logging and inspection.
pub trait ContentFlattener: Send + Sync {
    /// Return the text representation of `content`.
    fn flatten(&self, content: &MessageContent) -> String;
}
