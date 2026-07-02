use crate::api::provider::types::ExecutionMode;
use serde::{Deserialize, Serialize};

/// Configuration for execution
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// Maximum tokens to generate
    pub max_tokens_per_call: u32,

    /// Timeout per execution step
    pub timeout_per_step: u64, // milliseconds

    /// Enable prompt caching
    pub cache_enabled: bool,

    /// Enable streaming responses
    pub streaming_enabled: bool,

    /// Execution mode (Async, LongRunning, Streaming)
    pub execution_mode: ExecutionMode,
}
