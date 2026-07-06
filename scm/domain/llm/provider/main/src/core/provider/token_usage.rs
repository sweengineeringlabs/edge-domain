//! Constructors and accessors for [`TokenUsage`].

use crate::api::TokenUsage;

impl TokenUsage {
    /// Create a new token usage record
    pub fn new(
        prompt_tokens: u32,
        completion_tokens: u32,
        cache_read_input_tokens: u32,
        cache_creation_input_tokens: u32,
    ) -> Self {
        let total_tokens = Self::sum(prompt_tokens, completion_tokens);
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

    /// Add prompt and completion token counts.
    fn sum(prompt_tokens: u32, completion_tokens: u32) -> u32 {
        prompt_tokens + completion_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_computes_total_tokens() {
        let usage = TokenUsage::new(10, 20, 0, 0);
        assert_eq!(usage.total_tokens, 30);
    }

    /// @covers: total_with_cache
    #[test]
    fn test_total_with_cache_includes_cache_fields() {
        let usage = TokenUsage::new(10, 20, 5, 2);
        assert_eq!(usage.total_with_cache(), 37);
    }

    /// @covers: cache_hit
    #[test]
    fn test_cache_hit_false_when_no_cache_reads() {
        let usage = TokenUsage::new(10, 20, 0, 0);
        assert!(!usage.cache_hit());
    }

    /// @covers: sum
    #[test]
    fn test_sum_adds_both_values() {
        assert_eq!(TokenUsage::sum(10, 20), 30);
    }
}
