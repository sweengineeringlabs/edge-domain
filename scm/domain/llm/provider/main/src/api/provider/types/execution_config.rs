use crate::api::provider::types::ExecutionMode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for execution
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::ExecutionConfig;
    use crate::api::provider::types::ExecutionMode;

    #[test]
    fn test_timeout_converts_to_duration() {
        let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
        assert_eq!(config.timeout().as_millis(), 30_000);
    }

    #[test]
    fn test_supports_streaming_true_when_enabled() {
        let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
        assert!(config.supports_streaming());
    }

    #[test]
    fn test_supports_streaming_false_when_disabled() {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Streaming);
        assert!(!config.supports_streaming());
    }

    #[test]
    fn test_execution_config_serde_roundtrip() {
        let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
        let json = serde_json::to_string(&config).expect("serialize");
        let back: ExecutionConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.max_tokens_per_call, 4096);
    }
}
