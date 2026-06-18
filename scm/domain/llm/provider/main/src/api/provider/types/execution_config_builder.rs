//! `ExecutionConfigBuilder` — fluent builder for [`ExecutionConfig`].

use crate::api::provider::types::{ExecutionConfig, ExecutionMode};

/// Fluent builder for [`ExecutionConfig`].
#[derive(Clone, Debug)]
pub struct ExecutionConfigBuilder {
    max_tokens_per_call: u32,
    timeout_per_step: u64,
    cache_enabled: bool,
    streaming_enabled: bool,
    execution_mode: ExecutionMode,
}

impl Default for ExecutionConfigBuilder {
    fn default() -> Self {
        Self {
            max_tokens_per_call: 4096,
            timeout_per_step: 30_000,
            cache_enabled: false,
            streaming_enabled: false,
            execution_mode: ExecutionMode::Async,
        }
    }
}

impl ExecutionConfigBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the per-call token cap.
    pub fn max_tokens_per_call(mut self, value: u32) -> Self {
        self.max_tokens_per_call = value;
        self
    }

    /// Set the per-step timeout in milliseconds.
    pub fn timeout_per_step(mut self, value: u64) -> Self {
        self.timeout_per_step = value;
        self
    }

    /// Enable or disable prompt caching.
    pub fn cache_enabled(mut self, value: bool) -> Self {
        self.cache_enabled = value;
        self
    }

    /// Enable or disable streaming.
    pub fn streaming_enabled(mut self, value: bool) -> Self {
        self.streaming_enabled = value;
        self
    }

    /// Set the execution mode.
    pub fn execution_mode(mut self, value: ExecutionMode) -> Self {
        self.execution_mode = value;
        self
    }

    /// Build the [`ExecutionConfig`].
    pub fn build(self) -> ExecutionConfig {
        ExecutionConfig::new(
            self.max_tokens_per_call,
            self.timeout_per_step,
            self.cache_enabled,
            self.streaming_enabled,
            self.execution_mode,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionConfigBuilder;
    use crate::api::provider::types::ExecutionMode;

    #[test]
    fn test_execution_config_builder_applies_overrides() {
        let config = ExecutionConfigBuilder::new()
            .max_tokens_per_call(2048)
            .timeout_per_step(10_000)
            .execution_mode(ExecutionMode::Streaming)
            .streaming_enabled(true)
            .build();
        assert_eq!(config.max_tokens_per_call, 2048);
        assert_eq!(config.timeout_per_step, 10_000);
        assert!(config.supports_streaming());
    }

    #[test]
    fn test_execution_config_builder_defaults() {
        let config = ExecutionConfigBuilder::new().build();
        assert_eq!(config.max_tokens_per_call, 4096);
        assert_eq!(config.execution_mode, ExecutionMode::Async);
    }
}
