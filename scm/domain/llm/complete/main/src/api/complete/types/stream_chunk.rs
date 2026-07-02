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
