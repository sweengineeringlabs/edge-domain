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

#[cfg(test)]
mod tests {
    use super::StreamChunk;
    use crate::api::provider::types::{FinishReason, StreamDelta};

    #[test]
    fn test_is_terminal_true_with_finish_reason() {
        let chunk = StreamChunk::new("c1".to_string(), StreamDelta::empty(), Some(FinishReason::Stop));
        assert!(chunk.is_terminal());
    }

    #[test]
    fn test_is_terminal_false_without_finish_reason() {
        let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
        assert!(!chunk.is_terminal());
    }

    #[test]
    fn test_stream_chunk_serde_roundtrip() {
        let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
        let json = serde_json::to_string(&chunk).expect("serialize");
        let back: StreamChunk = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.id, "c1");
    }
}
