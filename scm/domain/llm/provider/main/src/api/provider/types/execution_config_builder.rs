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

    /// @covers: build
    #[test]
    fn test_execution_config_builder_defaults() {
        let config = ExecutionConfigBuilder::new().build();
        assert_eq!(config.max_tokens_per_call, 4096);
        assert_eq!(config.execution_mode, ExecutionMode::Async);
    }

    /// @covers: max_tokens_per_call
    #[test]
    fn test_max_tokens_per_call() {
        let c = ExecutionConfigBuilder::new().max_tokens_per_call(512).build();
        assert_eq!(c.max_tokens_per_call, 512);
    }

    /// @covers: timeout_per_step
    #[test]
    fn test_timeout_per_step() {
        let c = ExecutionConfigBuilder::new().timeout_per_step(5000).build();
        assert_eq!(c.timeout_per_step, 5000);
    }

    /// @covers: cache_enabled
    #[test]
    fn test_cache_enabled() {
        let c = ExecutionConfigBuilder::new().cache_enabled(true).build();
        assert!(c.cache_enabled);
    }

    /// @covers: streaming_enabled
    #[test]
    fn test_streaming_enabled() {
        let c = ExecutionConfigBuilder::new().streaming_enabled(true).build();
        assert!(c.streaming_enabled);
    }

    /// @covers: execution_mode
    #[test]
    fn test_execution_mode() {
        let c = ExecutionConfigBuilder::new().execution_mode(ExecutionMode::Streaming).build();
        assert_eq!(c.execution_mode, ExecutionMode::Streaming);
    }
}
