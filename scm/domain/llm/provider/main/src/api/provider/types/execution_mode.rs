use serde::{Deserialize, Serialize};

/// How a skill/tool executes
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Standard async/await (completes in single request/response cycle)
    #[default]
    Async,

    /// Long-running (may require polling or callback)
    LongRunning,

    /// Streaming (yields partial results incrementally)
    Streaming,
}
