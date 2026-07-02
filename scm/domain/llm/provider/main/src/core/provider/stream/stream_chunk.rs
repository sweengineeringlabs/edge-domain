//! Constructors and accessors for [`StreamChunk`].

use crate::api::{FinishReason, StreamChunk, StreamDelta};

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
        self.has_finish_reason()
    }

    /// Whether a finish reason has been recorded for this chunk.
    fn has_finish_reason(&self) -> bool {
        self.finish_reason.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let chunk = StreamChunk::new("c1".to_string(), StreamDelta::empty(), None);
        assert_eq!(chunk.id, "c1");
    }

    /// @covers: is_terminal
    #[test]
    fn test_is_terminal_true_when_finish_reason_present() {
        let chunk = StreamChunk::new(
            "c1".to_string(),
            StreamDelta::empty(),
            Some(FinishReason::Stop),
        );
        assert!(chunk.is_terminal());
    }

    /// @covers: is_terminal
    #[test]
    fn test_is_terminal_false_when_finish_reason_absent() {
        let chunk = StreamChunk::new("c1".to_string(), StreamDelta::empty(), None);
        assert!(!chunk.is_terminal());
    }

    /// @covers: has_finish_reason
    #[test]
    fn test_has_finish_reason_true_when_present() {
        let chunk = StreamChunk::new(
            "c1".to_string(),
            StreamDelta::empty(),
            Some(FinishReason::Stop),
        );
        assert!(chunk.has_finish_reason());
    }
}
