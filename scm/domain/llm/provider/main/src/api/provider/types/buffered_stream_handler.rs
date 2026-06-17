//! `BufferedStreamHandler` — reference [`StreamHandler`](crate::api::provider::traits::StreamHandler) implementation.

use crate::api::provider::types::{StreamChunk, ToolCallDelta};

/// Reference stream handler that accumulates deltas into a text buffer and a
/// pending tool call.
#[derive(Clone, Debug, Default)]
pub struct BufferedStreamHandler {
    pub(crate) buffer: String,
    pub(crate) pending: Option<ToolCallDelta>,
    pub(crate) queued: Vec<StreamChunk>,
}

impl BufferedStreamHandler {
    /// Construct an empty stream handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current accumulated text.
    pub fn text(&self) -> &str {
        &self.buffer
    }
}
