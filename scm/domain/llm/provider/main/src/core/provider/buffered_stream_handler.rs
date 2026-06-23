//! `StreamHandler` impl for `BufferedStreamHandler`.

use crate::api::StreamHandler;
use crate::api::{BufferedStreamHandler, FinishReason, StreamChunk, StreamDelta, ToolCallDelta};

impl StreamHandler for BufferedStreamHandler {
    fn next_chunk(&mut self) -> Option<StreamChunk> {
        if self.queued.is_empty() {
            return None;
        }
        Some(self.queued.remove(0))
    }

    fn accumulate(&mut self, delta: StreamDelta) {
        match delta {
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
    }

    fn pending_tool_call(&self) -> Option<ToolCallDelta> {
        self.pending.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate_appends_text() {
        let mut handler = BufferedStreamHandler::new();
        handler.accumulate(StreamDelta::text("hello ".to_string()));
        handler.accumulate(StreamDelta::text("world".to_string()));
        assert_eq!(handler.text(), "hello world");
    }

    #[test]
    fn test_next_chunk_drains_queue() {
        let mut handler = BufferedStreamHandler::new();
        handler.accumulate(StreamDelta::text("hi".to_string()));
        let chunk = handler.next_chunk();
        assert!(chunk.is_some(), "first next_chunk should return Some after accumulate");
        let chunk_val = chunk.unwrap();
        assert!(!chunk_val.id.is_empty(), "chunk should have non-empty id");
        assert!(!chunk_val.delta.is_empty(), "chunk should contain delta data");
        let empty = handler.next_chunk();
        assert!(empty.is_none(), "second next_chunk should return None after draining");
    }

    #[test]
    fn test_pending_tool_call_tracks_delta() {
        let mut handler = BufferedStreamHandler::new();
        handler.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]));
        let pending = handler.pending_tool_call();
        assert!(pending.is_some(), "pending_tool_call should return Some after tool_calls accumulation");
        let tool_call = pending.unwrap();
        assert_eq!(tool_call.index, 0, "tool call index should be preserved");
    }
}
