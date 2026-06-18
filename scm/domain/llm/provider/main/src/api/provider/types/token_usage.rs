use serde::{Deserialize, Serialize};

/// Token usage tracking across prompt, completion, and caching
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Tokens in the prompt
    pub prompt_tokens: u32,

    /// Tokens in the completion
    pub completion_tokens: u32,

    /// Total tokens (prompt + completion)
    pub total_tokens: u32,

    /// Tokens read from cache (Anthropic prompt caching)
    pub cache_read_input_tokens: u32,

    /// Tokens written to cache
    pub cache_creation_input_tokens: u32,
}

impl TokenUsage {
    /// Create a new token usage record
    pub fn new(
        prompt_tokens: u32,
        completion_tokens: u32,
        cache_read_input_tokens: u32,
        cache_creation_input_tokens: u32,
    ) -> Self {
        let total_tokens = prompt_tokens + completion_tokens;
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            cache_read_input_tokens,
            cache_creation_input_tokens,
        }
    }

    /// Total tokens including cache operations
    pub fn total_with_cache(&self) -> u32 {
        self.total_tokens + self.cache_read_input_tokens + self.cache_creation_input_tokens
    }

    /// Check if cache was hit (any reads)
    pub fn cache_hit(&self) -> bool {
        self.cache_read_input_tokens > 0
    }
}
