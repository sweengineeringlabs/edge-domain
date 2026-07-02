use serde::{Deserialize, Serialize};

/// Metadata describing an LLM model available on a provider.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelInfo {
    /// Model identifier (e.g. `"gpt-4o"`).
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Provider name (e.g. `"openai"`).
    pub provider: String,
    /// Maximum tokens the model accepts (prompt + completion).
    pub context_window: u32,
    /// Whether the model can process image inputs.
    pub supports_vision: bool,
    /// Whether the model supports tool/function calling.
    pub supports_function_calling: bool,
    /// Whether the model supports streaming output.
    pub supports_streaming: bool,
}
