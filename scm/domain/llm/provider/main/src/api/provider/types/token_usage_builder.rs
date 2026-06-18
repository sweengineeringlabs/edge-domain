//! `TokenUsageBuilder` — fluent builder for [`TokenUsage`].

use crate::api::provider::types::TokenUsage;

/// Fluent builder for [`TokenUsage`].
#[derive(Clone, Debug, Default)]
pub struct TokenUsageBuilder {
    prompt_tokens: u32,
    completion_tokens: u32,
    cache_read_input_tokens: u32,
    cache_creation_input_tokens: u32,
}

impl TokenUsageBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set prompt tokens.
    pub fn prompt_tokens(mut self, value: u32) -> Self {
        self.prompt_tokens = value;
        self
    }

    /// Set completion tokens.
    pub fn completion_tokens(mut self, value: u32) -> Self {
        self.completion_tokens = value;
        self
    }

    /// Set cache-read input tokens.
    pub fn cache_read_input_tokens(mut self, value: u32) -> Self {
        self.cache_read_input_tokens = value;
        self
    }

    /// Set cache-creation input tokens.
    pub fn cache_creation_input_tokens(mut self, value: u32) -> Self {
        self.cache_creation_input_tokens = value;
        self
    }

    /// Build the [`TokenUsage`], computing the total.
    pub fn build(self) -> TokenUsage {
        TokenUsage::new(
            self.prompt_tokens,
            self.completion_tokens,
            self.cache_read_input_tokens,
            self.cache_creation_input_tokens,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TokenUsageBuilder;

    #[test]
    fn test_token_usage_builder_computes_total() {
        let usage = TokenUsageBuilder::new()
            .prompt_tokens(100)
            .completion_tokens(50)
            .cache_read_input_tokens(20)
            .build();
        assert_eq!(usage.total_tokens, 150);
        assert!(usage.cache_hit());
    }

    /// @covers: build
    #[test]
    fn test_token_usage_builder_defaults_zero() {
        let usage = TokenUsageBuilder::new().build();
        assert_eq!(usage.total_tokens, 0);
        assert!(!usage.cache_hit());
    }

    /// @covers: prompt_tokens
    #[test]
    fn test_prompt_tokens() {
        let u = TokenUsageBuilder::new().prompt_tokens(100).build();
        assert_eq!(u.prompt_tokens, 100);
    }

    /// @covers: completion_tokens
    #[test]
    fn test_completion_tokens() {
        let u = TokenUsageBuilder::new().completion_tokens(50).build();
        assert_eq!(u.completion_tokens, 50);
    }

    /// @covers: cache_read_input_tokens
    #[test]
    fn test_cache_read_input_tokens() {
        let u = TokenUsageBuilder::new().cache_read_input_tokens(20).build();
        assert_eq!(u.cache_read_input_tokens, 20);
    }

    /// @covers: cache_creation_input_tokens
    #[test]
    fn test_cache_creation_input_tokens() {
        let u = TokenUsageBuilder::new().cache_creation_input_tokens(10).build();
        assert_eq!(u.cache_creation_input_tokens, 10);
    }
}
