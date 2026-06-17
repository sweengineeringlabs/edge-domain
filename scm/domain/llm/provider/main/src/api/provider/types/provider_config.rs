use serde::{Deserialize, Serialize};

/// Provider configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Model name/ID
    pub model: String,

    /// Temperature (0.0..=2.0)
    pub temperature: f32,

    /// API base URL
    pub api_base: Option<String>,

    /// Maximum context window
    pub max_context_tokens: u32,

    /// Supports vision/image input
    pub supports_vision: bool,

    /// Supports function calling
    pub supports_functions: bool,

    /// Supports streaming
    pub supports_streaming: bool,
}

impl ProviderConfig {
    /// Create a new provider config
    pub fn new(model: String, temperature: f32, max_context_tokens: u32) -> Self {
        Self {
            model,
            temperature,
            api_base: None,
            max_context_tokens,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
        }
    }
}
