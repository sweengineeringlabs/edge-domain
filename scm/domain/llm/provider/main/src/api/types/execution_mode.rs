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
