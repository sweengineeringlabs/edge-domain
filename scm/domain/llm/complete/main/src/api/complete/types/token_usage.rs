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
