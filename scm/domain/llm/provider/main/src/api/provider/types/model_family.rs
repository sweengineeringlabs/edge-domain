use serde::{Deserialize, Serialize};

/// LLM model family/provider
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
    #[serde(rename = "other")]
    Other,
}

#[cfg(test)]
mod tests {
    use super::ModelFamily;

    #[test]
    fn test_model_family_variants_distinct() {
        assert_ne!(ModelFamily::Anthropic, ModelFamily::OpenAI);
        assert_ne!(ModelFamily::Google, ModelFamily::OpenSource);
    }

    #[test]
    fn test_model_family_equality() {
        assert_eq!(ModelFamily::Anthropic, ModelFamily::Anthropic);
    }

    #[test]
    fn test_model_family_serde_roundtrip() {
        let json = serde_json::to_string(&ModelFamily::Google).expect("serialize");
        let back: ModelFamily = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, ModelFamily::Google);
    }
}
