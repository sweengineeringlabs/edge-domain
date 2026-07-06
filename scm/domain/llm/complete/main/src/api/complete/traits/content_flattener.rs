//! `ContentFlattener` — flattens structured [`MessageContent`] to plain text.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{FlattenRequest, FlattenResponse};

/// Reduces a [`MessageContent`](crate::api::complete::types::MessageContent) to a plain-text
/// string for logging and inspection.
pub trait ContentFlattener: Send + Sync {
    /// Return the text representation of the given content.
    fn flatten(&self, req: FlattenRequest<'_>) -> Result<FlattenResponse, CompleteError>;
}
