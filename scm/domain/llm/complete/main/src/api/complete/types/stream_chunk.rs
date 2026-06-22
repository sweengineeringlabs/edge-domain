use serde::{Deserialize, Serialize};

use crate::api::complete::types::{FinishReason, StreamDelta};

/// A single chunk emitted by a streaming completion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamChunk {
    /// Provider-assigned chunk identifier.
    pub id: String,
    /// Incremental payload for this chunk.
    pub delta: Box<StreamDelta>,
    /// Present only on the final chunk.
    pub finish_reason: Option<FinishReason>,
}

impl StreamChunk {
    /// Construct a mid-stream chunk (no finish reason).
    pub fn partial(id: impl Into<String>, delta: StreamDelta) -> Self {
        Self {
            id: id.into(),
            delta: Box::new(delta),
            finish_reason: None,
        }
    }

    /// Construct the terminal chunk that carries a finish reason.
    pub fn terminal(id: impl Into<String>, delta: StreamDelta, reason: FinishReason) -> Self {
        Self {
            id: id.into(),
            delta: Box::new(delta),
            finish_reason: Some(reason),
        }
    }
}
