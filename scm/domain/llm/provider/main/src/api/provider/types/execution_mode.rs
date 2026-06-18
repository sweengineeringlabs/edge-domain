use serde::{Deserialize, Serialize};

/// How a skill/tool executes
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Standard async/await (completes in single request/response cycle)
    Async,

    /// Long-running (may require polling or callback)
    LongRunning,

    /// Streaming (yields partial results incrementally)
    Streaming,
}

impl ExecutionMode {
    /// Check if this mode supports streaming
    pub fn is_streaming(&self) -> bool {
        matches!(self, ExecutionMode::Streaming)
    }

    /// Check if this mode is asynchronous
    pub fn is_async(&self) -> bool {
        !matches!(self, ExecutionMode::LongRunning)
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionMode;

    #[test]
    fn test_is_streaming_true_for_streaming() {
        assert!(ExecutionMode::Streaming.is_streaming());
    }

    #[test]
    fn test_is_streaming_false_for_async() {
        assert!(!ExecutionMode::Async.is_streaming());
    }

    #[test]
    fn test_is_async_true_for_async() {
        assert!(ExecutionMode::Async.is_async());
    }

    #[test]
    fn test_execution_mode_serde_roundtrip() {
        let json = serde_json::to_string(&ExecutionMode::LongRunning).expect("serialize");
        let back: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, ExecutionMode::LongRunning);
    }

    #[test]
    fn test_execution_mode_equality() {
        assert_eq!(ExecutionMode::Async, ExecutionMode::Async);
        assert_ne!(ExecutionMode::Async, ExecutionMode::Streaming);
    }
}
