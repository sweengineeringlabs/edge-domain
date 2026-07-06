use crate::api::provider::types::{FinishReason, StreamDelta};
use serde::{Deserialize, Serialize};

/// Chunk of a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Chunk ID
    pub id: String,

    /// Delta content
    pub delta: StreamDelta, // @allow: api_field_type_purity — data aggregate, no trait exists

    /// Reason stream ended (if terminal)
    pub finish_reason: Option<FinishReason>,
}
