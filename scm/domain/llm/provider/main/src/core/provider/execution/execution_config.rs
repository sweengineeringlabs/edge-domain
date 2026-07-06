//! Constructors and `ConfigSection` impl for [`ExecutionConfig`].

use std::time::Duration;

use swe_edge_configbuilder::ConfigSection;

use crate::api::ExecutionConfig;
use crate::api::ExecutionMode;

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
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — TOML section key for this type
        "llm.execution"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        assert_eq!(config.max_tokens_per_call, 4096);
        assert_eq!(config.execution_mode, ExecutionMode::Async);
    }

    /// @covers: timeout
    #[test]
    fn test_timeout_converts_millis_to_duration() {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        assert_eq!(config.timeout(), Duration::from_millis(30_000));
    }

    /// @covers: supports_streaming
    #[test]
    fn test_supports_streaming_false_for_async_mode() {
        let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
        assert!(!config.supports_streaming());
    }

    /// @covers: supports_streaming
    #[test]
    fn test_supports_streaming_true_for_streaming_mode() {
        let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
        assert!(config.supports_streaming());
    }

    /// @covers: section_name
    #[test]
    fn test_section_name_is_llm_execution() {
        assert_eq!(ExecutionConfig::section_name(), "llm.execution");
    }
}
