//! Constructor for [`TokenUsage`].

use crate::api::TokenUsage;

impl TokenUsage {
    /// Construct a token usage record.
    ///
    /// `total_tokens` is clamped up to at least `prompt_tokens + completion_tokens`
    /// so the record can never under-report total consumption.
    pub fn new(
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
        cache_read_input_tokens: u32,
        cache_creation_input_tokens: u32,
    ) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: Self::clamped_total(total_tokens, prompt_tokens, completion_tokens),
            cache_read_input_tokens,
            cache_creation_input_tokens,
        }
    }

    /// Raise `total_tokens` to at least `prompt_tokens + completion_tokens`.
    fn clamped_total(total_tokens: u32, prompt_tokens: u32, completion_tokens: u32) -> u32 {
        total_tokens.max(prompt_tokens + completion_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let usage = TokenUsage::new(10, 20, 30, 5, 2);
        assert_eq!(usage.total_tokens, 30);
        assert_eq!(usage.cache_read_input_tokens, 5);
    }

    /// @covers: clamped_total
    #[test]
    fn test_clamped_total_raises_understated_total() {
        assert_eq!(TokenUsage::clamped_total(0, 10, 20), 30);
    }
}
