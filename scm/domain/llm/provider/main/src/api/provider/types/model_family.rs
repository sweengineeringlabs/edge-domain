use serde::{Deserialize, Serialize};

/// LLM model family/provider
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ModelFamily {
    /// Anthropic Claude models
    #[serde(rename = "anthropic")]
    Anthropic,

    /// OpenAI models (GPT-3, GPT-4, etc.)
    #[serde(rename = "openai")]
    OpenAI,

    /// Google models (PaLM, Gemini, etc.)
    #[serde(rename = "google")]
    Google,

    /// Open-source models (LLaMA, Mistral, etc.)
    #[serde(rename = "open_source")]
    OpenSource,

    /// Other providers
    #[default]
    #[serde(rename = "other")]
    Other,
}
