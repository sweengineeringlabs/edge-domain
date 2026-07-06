//! Constructors for [`StreamChunk`].

use crate::api::{FinishReason, StreamChunk, StreamDelta};

impl StreamChunk {
    /// Construct a mid-stream chunk (no finish reason).
    pub fn partial(id: impl Into<String>, delta: StreamDelta) -> Self {
        Self::build(id.into(), delta, None)
    }

    /// Construct the terminal chunk that carries a finish reason.
    pub fn terminal(id: impl Into<String>, delta: StreamDelta, reason: FinishReason) -> Self {
        Self::build(id.into(), delta, Some(reason))
    }

    /// Shared constructor for the `partial`/`terminal` factory methods.
    fn build(id: String, delta: StreamDelta, finish_reason: Option<FinishReason>) -> Self {
        Self {
            id,
            delta: Box::new(delta),
            finish_reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: partial
    #[test]
    fn test_partial_has_no_finish_reason() {
        let chunk = StreamChunk::partial("chunk-1", StreamDelta::text("hi"));
        assert_eq!(chunk.finish_reason, None);
    }

    /// @covers: terminal
    #[test]
    fn test_terminal_sets_finish_reason() {
        let chunk = StreamChunk::terminal("chunk-1", StreamDelta::empty(), FinishReason::Stop);
        assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
    }

    /// @covers: build
    #[test]
    fn test_build_sets_id_and_delta() {
        let chunk = StreamChunk::build("c-1".to_string(), StreamDelta::text("hi"), None);
        assert_eq!(chunk.id, "c-1");
        assert_eq!(chunk.delta.content, Some("hi".to_string()));
    }
}
