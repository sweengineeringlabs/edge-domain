use crate::api::provider::types::ExecutionMode;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use swe_edge_configbuilder::ConfigSection;

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

impl ExecutionConfig {
    /// Create a new execution config
    pub fn new(
        max_tokens_per_call: u32,
        timeout_per_step: u64,
        cache_enabled: bool,
        streaming_enabled: bool,
        execution_mode: ExecutionMode,
    ) -> Self {
        Self {
            max_tokens_per_call,
            timeout_per_step,
            cache_enabled,
            streaming_enabled,
            execution_mode,
        }
    }

    /// Get timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_per_step)
    }

    /// Check if streaming is available
    pub fn supports_streaming(&self) -> bool {
        self.streaming_enabled && self.execution_mode.is_streaming()
    }
}

impl ConfigSection for ExecutionConfig {
    fn section_name() -> &'static str { // @allow: no_stub_fn_bodies — TOML section key for this type
        "llm.execution"
    }
}
