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
        if let Some(text) = &delta.content {
            self.buffer.push_str(text);
        }
        if let Some(call) = delta.tool_calls.into_iter().next() {
            self.pending = Some(call);
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
        assert!(handler.next_chunk().is_some());
        assert!(handler.next_chunk().is_none());
    }

    #[test]
    fn test_pending_tool_call_tracks_delta() {
        let mut handler = BufferedStreamHandler::new();
        handler.accumulate(StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]));
        assert!(handler.pending_tool_call().is_some());
    }
}
