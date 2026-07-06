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
