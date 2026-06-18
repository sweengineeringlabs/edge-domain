use crate::api::provider::types::ToolCallDelta;
use serde::{Deserialize, Serialize};

/// Incremental update in a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDelta {
    /// Text content (if text streaming)
    pub content: Option<String>,

    /// Tool call deltas (if tool streaming)
    pub tool_calls: Vec<ToolCallDelta>,
}

impl StreamDelta {
    /// Create a text delta
    pub fn text(content: String) -> Self {
        Self {
            content: Some(content),
            tool_calls: vec![],
        }
    }

    /// Create a tool call delta
    pub fn tool_calls(calls: Vec<ToolCallDelta>) -> Self {
        Self {
            content: None,
            tool_calls: calls,
        }
    }

    /// Create an empty delta
    pub fn empty() -> Self {
        Self {
            content: None,
            tool_calls: vec![],
        }
    }

    /// Check if delta has content
    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.tool_calls.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::StreamDelta;
    use crate::api::provider::types::ToolCallDelta;

    #[test]
    fn test_text_carries_content() {
        let delta = StreamDelta::text("hello".to_string());
        assert_eq!(delta.content.as_deref(), Some("hello"));
        assert!(!delta.is_empty());
    }

    #[test]
    fn test_empty_is_empty() {
        assert!(StreamDelta::empty().is_empty());
    }

    #[test]
    fn test_tool_calls_carry_deltas() {
        let delta = StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]);
        assert_eq!(delta.tool_calls.len(), 1);
        assert!(!delta.is_empty());
    }

    #[test]
    fn test_stream_delta_serde_roundtrip() {
        let delta = StreamDelta::text("hi".to_string());
        let json = serde_json::to_string(&delta).expect("serialize");
        let back: StreamDelta = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.content.as_deref(), Some("hi"));
    }
}
