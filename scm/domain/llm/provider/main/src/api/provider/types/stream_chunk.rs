use crate::api::provider::types::{FinishReason, StreamDelta};
use serde::{Deserialize, Serialize};

/// Chunk of a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Chunk ID
    pub id: String,

    /// Delta content
    pub delta: StreamDelta,

    /// Reason stream ended (if terminal)
    pub finish_reason: Option<FinishReason>,
}

impl StreamChunk {
    /// Create a new stream chunk
    pub fn new(id: String, delta: StreamDelta, finish_reason: Option<FinishReason>) -> Self {
        Self {
            id,
            delta,
            finish_reason,
        }
    }

    /// Check if this chunk is terminal (stream finished)
    pub fn is_terminal(&self) -> bool {
        self.finish_reason.is_some()
    }
}
