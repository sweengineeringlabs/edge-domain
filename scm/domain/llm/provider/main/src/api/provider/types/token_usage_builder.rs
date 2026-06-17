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
