//! `StreamHandler` impl for `BufferedStreamHandler`.

use crate::api::StreamHandler;
use crate::api::{
    AccumulateRequest, BufferedStreamHandler, ExecutionError, FinishReason, NextChunkRequest,
    NextChunkResponse, PendingToolCallRequest, PendingToolCallResponse, StreamChunk, StreamDelta,
};

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

impl StreamHandler for BufferedStreamHandler {
    fn next_chunk(&mut self, _req: NextChunkRequest) -> Result<NextChunkResponse, ExecutionError> {
        if self.queued.is_empty() {
            return Ok(NextChunkResponse { chunk: None });
        }
        Ok(NextChunkResponse {
            chunk: Some(Box::new(self.queued.remove(0))),
        })
    }

    fn accumulate(&mut self, req: AccumulateRequest) -> Result<(), ExecutionError> {
        match req.delta {
            StreamDelta::Text(text) => self.buffer.push_str(&text),
            StreamDelta::ToolCalls(calls) => {
                if let Some(call) = calls.into_iter().next() {
                    self.pending = Some(call);
                }
            }
            StreamDelta::Empty => {}
        }
        let terminal = self.pending.is_none() && !self.buffer.is_empty();
        let chunk = StreamChunk::new(
            format!("chunk-{}", self.queued.len()),
            StreamDelta::text(self.buffer.clone()),
            terminal.then_some(FinishReason::Stop),
        );
        self.queued.push(chunk);
        Ok(())
    }

    fn pending_tool_call(
        &self,
        _req: PendingToolCallRequest,
    ) -> Result<PendingToolCallResponse, ExecutionError> {
        Ok(PendingToolCallResponse {
            tool_call: self.pending.clone().map(Box::new),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ToolCallDelta;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_text() {
        assert_eq!(BufferedStreamHandler::new().text(), "");
    }

    /// @covers: text
    #[test]
    fn test_text_reflects_accumulated_buffer() {
        let mut handler = BufferedStreamHandler::new();
        handler
            .accumulate(AccumulateRequest {
                delta: StreamDelta::text("hi".to_string()),
            })
            .expect("accumulate ok");
        assert_eq!(handler.text(), "hi");
    }

    #[test]
    fn test_accumulate_appends_text() {
        let mut handler = BufferedStreamHandler::new();
        handler
            .accumulate(AccumulateRequest {
                delta: StreamDelta::text("hello ".to_string()),
            })
            .expect("accumulate ok");
        handler
            .accumulate(AccumulateRequest {
                delta: StreamDelta::text("world".to_string()),
            })
            .expect("accumulate ok");
        assert_eq!(handler.text(), "hello world");
    }

    #[test]
    fn test_next_chunk_drains_queue() {
        let mut handler = BufferedStreamHandler::new();
        handler
            .accumulate(AccumulateRequest {
                delta: StreamDelta::text("hi".to_string()),
            })
            .expect("accumulate ok");
        let chunk = handler
            .next_chunk(NextChunkRequest)
            .expect("next_chunk ok")
            .chunk;
        assert!(
            chunk.is_some(),
            "first next_chunk should return Some after accumulate"
        );
        let chunk_val = chunk.unwrap();
        assert!(!chunk_val.id.is_empty(), "chunk should have non-empty id");
        assert!(
            !chunk_val.delta.is_empty(),
            "chunk should contain delta data"
        );
        let empty = handler
            .next_chunk(NextChunkRequest)
            .expect("next_chunk ok")
            .chunk;
        assert!(
            empty.is_none(),
            "second next_chunk should return None after draining"
        );
    }

    #[test]
    fn test_pending_tool_call_tracks_delta() {
        let mut handler = BufferedStreamHandler::new();
        handler
            .accumulate(AccumulateRequest {
                delta: StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]),
            })
            .expect("accumulate ok");
        let pending = handler
            .pending_tool_call(PendingToolCallRequest)
            .expect("pending_tool_call ok")
            .tool_call;
        assert!(
            pending.is_some(),
            "pending_tool_call should return Some after tool_calls accumulation"
        );
        let tool_call = pending.unwrap();
        assert_eq!(tool_call.index, 0, "tool call index should be preserved");
    }
}
