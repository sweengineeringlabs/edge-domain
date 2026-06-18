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

#[cfg(test)]
mod tests {
    use super::TokenUsage;

    #[test]
    fn test_new_sums_prompt_and_completion() {
        let usage = TokenUsage::new(100, 50, 0, 0);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_cache_hit_true_with_reads() {
        let usage = TokenUsage::new(100, 50, 20, 0);
        assert!(usage.cache_hit());
    }

    #[test]
    fn test_cache_hit_false_without_reads() {
        let usage = TokenUsage::new(100, 50, 0, 10);
        assert!(!usage.cache_hit());
    }

    #[test]
    fn test_total_with_cache_includes_cache_ops() {
        let usage = TokenUsage::new(100, 50, 20, 10);
        assert_eq!(usage.total_with_cache(), 180);
    }

    #[test]
    fn test_token_usage_serde_roundtrip() {
        let usage = TokenUsage::new(1, 2, 3, 4);
        let json = serde_json::to_string(&usage).expect("serialize");
        let back: TokenUsage = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(usage, back);
    }
}
