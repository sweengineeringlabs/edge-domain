use serde::{Deserialize, Serialize};

/// Token consumption breakdown for a single completion.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    /// Tokens in the prompt.
    pub prompt_tokens: u32,
    /// Tokens in the generated completion.
    pub completion_tokens: u32,
    /// Total tokens consumed.
    pub total_tokens: u32,
    /// Tokens served from the provider's prompt cache.
    pub cache_read_input_tokens: u32,
    /// Tokens written into the provider's prompt cache.
    pub cache_creation_input_tokens: u32,
}

impl TokenUsage {
    /// Construct a token usage record.
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
            total_tokens,
            cache_read_input_tokens,
            cache_creation_input_tokens,
        }
    }
}
